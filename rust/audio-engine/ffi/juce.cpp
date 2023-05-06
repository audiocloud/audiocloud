#include <memory>
#include <juce_audio_devices/juce_audio_devices.h>
#include <juce_audio_formats/juce_audio_formats.h>

std::unique_ptr<juce::AudioFormatManager> create_format_manager() {
    auto format_manager = std::make_unique<juce::AudioFormatManager>();
    format_manager->registerBasicFormats();
    return format_manager;
}

struct RustAudioIoCallback : public juce::AudioIODeviceCallback {
    void (*rust_callback)(
            void *data,
            const float *const *, int,
            float *const *, int,
            int){};

    void *data{};

    void audioDeviceIOCallbackWithContext(const float *const *inputChannelData,
                                          int numInputChannels,
                                          float *const *outputChannelData,
                                          int numOutputChannels,
                                          int numSamples,
                                          const juce::AudioIODeviceCallbackContext &context) override {
        this->rust_callback(this->data, inputChannelData, numInputChannels,   // input
                            outputChannelData, numOutputChannels, // output
                            numSamples);
    }

    void audioDeviceAboutToStart(juce::AudioIODevice *device) override {}

    void audioDeviceStopped() override {}

    void audioDeviceError(const juce::String &errorMessage) override {
        std::cerr << "Audio device error: " << errorMessage << std::endl;
    }
};

std::unique_ptr<juce::AudioDeviceManager> device_manager = std::make_unique<juce::AudioDeviceManager>();
std::map<uint32_t, std::unique_ptr<juce::AudioIODevice>> audio_devices;
std::atomic_int32_t audio_device_id{0};

extern "C" {
int32_t create_audio_device(const char *type_name,
                            const char *input_name, const char *output_name,
                            const int input_channel_count, const int output_channel_count,
                            const int sample_rate,
                            const int buffer_size) {
    auto our_device_id = audio_device_id.fetch_add(1);

    try {
        std::cout << "create_audio_device: " << type_name << " " << input_name << " " << output_name << " for device "
                  << our_device_id << std::endl;
        std::cout << input_channel_count << " x " << output_channel_count << " @ " << sample_rate << "Hz" << std::endl;

        for (auto dev_type: device_manager->getAvailableDeviceTypes()) {
            if (dev_type->getTypeName() != type_name) {
                continue;
            }

            auto audio_device = std::unique_ptr<juce::AudioIODevice>(dev_type->createDevice(output_name, input_name));
            if (audio_device == nullptr) {
                std::cerr << type_name << " device not found, listing devices ..." << std::endl;

                std::cerr << "input devices" << std::endl;
                for (auto &dev_name: dev_type->getDeviceNames(true)) {
                    std::cerr << " - " << dev_name << std::endl;
                }

                std::cerr << "output devices" << std::endl;
                for (auto &dev_name: dev_type->getDeviceNames(false)) {
                    std::cerr << " - " << dev_name << std::endl;
                }

                return -1;
            }

            juce::BigInteger input_channels, output_channels;

            input_channels.setRange(0, input_channel_count, true);
            output_channels.setRange(0, output_channel_count, true);

            auto error = audio_device->open(input_channels, output_channels, sample_rate, buffer_size);
            if (!error.isEmpty()) {
                std::cerr << error << std::endl;
                return -2;
            }

            audio_devices.emplace(our_device_id, std::move(audio_device));

            return our_device_id;
        }

        std::cerr << "No device type found" << std::endl;
    } catch (...) {
        std::cerr << "create_audio_device: exception" << std::endl;
    }

    return -3;
}

int32_t start_audio_device(int32_t device_id,
                           void (*rust_callback)(
                                   void *data,
                                   const float *const *, int,
                                   float *const *, int,
                                   int), void *data) {

    if (audio_devices.count(device_id) == 0) {
        std::cerr << "start_audio_mgr: device not found: " << device_id << std::endl;
        return -1;
    }

    if (!rust_callback) {
        std::cerr << "start_audio_mgr: callback not set" << std::endl;
        return -2;
    }


    auto callback = std::make_unique<RustAudioIoCallback>();
    callback->rust_callback = rust_callback;
    callback->data = data;

    audio_devices[device_id]->start(callback.release());
}

int32_t get_audio_device_latency(int32_t device_id) {
    if (audio_devices.count(device_id) == 0) {
        std::cerr << "get_audio_device_latency: device not found: " << device_id << std::endl;
        return -1;
    }

    auto &device = audio_devices[device_id];
    return static_cast<int32_t>(device->getInputLatencyInSamples() + device->getOutputLatencyInSamples());
}

void stop_audio_device(int32_t device_id) {
    if (audio_devices.count(device_id) == 0) {
        std::cerr << "stop_audio_device: device not found: " << device_id << std::endl;
        return;
    }

    audio_devices[device_id]->stop();
}

void delete_audio_device(int32_t device_id) {
    if (audio_devices.count(device_id) == 0) {
        std::cerr << "shutdown_audio_device: device not found: " << device_id << std::endl;
        return;
    }

    audio_devices.erase(device_id);
}

static std::unique_ptr<juce::ScopedJuceInitialiser_GUI> juce_gui = std::make_unique<juce::ScopedJuceInitialiser_GUI>();
static std::unique_ptr<juce::AudioFormatManager> format_manager = create_format_manager();
static std::unique_ptr<juce::TimeSliceThread> io_thread = std::make_unique<juce::TimeSliceThread>("io_thread");

typedef juce::BufferingAudioReader AudioFormatReader;

AudioFormatReader *create_file_reader(const char *path) {
    auto wd = juce::File::getCurrentWorkingDirectory();
    auto file = wd.getChildFile(juce::String(path));
    auto reader = format_manager->createReaderFor(file);
    if (reader == nullptr) {
        return nullptr;
    }

    return new juce::BufferingAudioReader(reader, *io_thread, 256 * 1024 * 1024);
}

void delete_file_reader(AudioFormatReader *reader) {
    delete reader;
}

int64_t file_reader_get_total_length(AudioFormatReader *reader) {
    return static_cast<int64_t>(reader->lengthInSamples);
}

uint32_t file_reader_get_channels(AudioFormatReader *reader) {
    return static_cast<uint32_t>(reader->numChannels);
}

uint32_t file_reader_get_sample_rate(AudioFormatReader *reader) {
    return static_cast<uint32_t>(reader->sampleRate);
}

int32_t file_reader_read_samples(AudioFormatReader *reader,
                                 float **buffers,
                                 int32_t num_channels,
                                 int64_t start_pos,
                                 int32_t num_samples,
                                 uint32_t timeout_ms) {
    reader->setReadTimeout(static_cast<int>(timeout_ms));

    return reader->read(buffers, num_channels, start_pos, num_samples) ? 1 : 0;
}

}