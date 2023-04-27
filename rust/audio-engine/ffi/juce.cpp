#include <memory>
#include <juce_audio_devices/juce_audio_devices.h>
#include <juce_audio_formats/juce_audio_formats.h>

struct AudioMgr {
    std::unique_ptr<juce::AudioDeviceManager> device_manager;
    std::unique_ptr<juce::AudioIODevice> device;
};

std::unique_ptr<juce::AudioFormatManager> create_format_manager() {
    auto format_manager = std::make_unique<juce::AudioFormatManager>();
    format_manager->registerBasicFormats();
    return format_manager;
}

static std::unique_ptr<AudioMgr> audio_mgr;

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

extern "C" {
int32_t init_audio_mgr(const char *type_name,
                       const char *input_name, const char *output_name,
                       const int input_channel_count, const int output_channel_count,
                       const int sample_rate,
                       const int buffer_size) {
    try {
        std::cout << "create_audio_mgr: " << type_name << " " << input_name << " " << output_name << std::endl;
        std::cout << input_channel_count << " x " << output_channel_count << " @ " << sample_rate << "Hz" << std::endl;

        auto device_manager = std::make_unique<juce::AudioDeviceManager>();

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

            audio_mgr = std::make_unique<AudioMgr>(AudioMgr{
                    std::move(device_manager),
                    std::move(audio_device)
            });

            return 0;
        }

        std::cerr << "No device type found" << std::endl;
    } catch (...) {
        std::cerr << "create_audio_mgr: exception" << std::endl;
    }

    return -3;
}

void audio_mgr_start(void (*rust_callback)(
        void *data,
        const float *const *, int,
        float *const *, int,
        int), void *data) {

    if (audio_mgr && audio_mgr->device && rust_callback) {
        auto callback = std::make_unique<RustAudioIoCallback>();
        callback->rust_callback = rust_callback;
        callback->data = data;

        audio_mgr->device->start(callback.release());
    }
}

uint32_t audio_mgr_get_latency() {
    if (audio_mgr && audio_mgr->device) {
        return audio_mgr->device->getInputLatencyInSamples() + audio_mgr->device->getOutputLatencyInSamples();
    }

    return 0;
}

void audio_mgr_stop(AudioMgr *mgr) {
    if (audio_mgr && audio_mgr->device) {
        audio_mgr->device->stop();
    }
}

static std::unique_ptr<juce::ScopedJuceInitialiser_GUI> juce_gui = std::make_unique<juce::ScopedJuceInitialiser_GUI>();
static std::unique_ptr<juce::AudioFormatManager> format_manager = create_format_manager();
static std::unique_ptr<juce::TimeSliceThread> io_thread = std::make_unique<juce::TimeSliceThread>("io_thread");


void juce_engine_shutdown() {
    format_manager.reset();
    juce::shutdownJuce_GUI();
}

juce::AudioFormatReader *create_file_reader(const char *path) {
    auto wd = juce::File::getCurrentWorkingDirectory();
    auto file = wd.getChildFile(juce::String(path));
    auto reader = format_manager->createReaderFor(file);
    if (reader == nullptr) {
        return nullptr;
    }

    return new juce::BufferingAudioReader(reader, *io_thread, 256 * 1024 * 1024);
}

void delete_file_reader(juce::AudioFormatReaderSource *reader) {
    delete reader;
}

int64_t file_reader_get_total_length(juce::AudioFormatReader *reader) {
    return static_cast<int64_t>(reader->lengthInSamples);
}

uint32_t file_reader_get_channels(juce::AudioFormatReader *reader) {
    return static_cast<uint32_t>(reader->numChannels);
}

uint32_t file_reader_get_sample_rate(juce::AudioFormatReader *reader) {
    return static_cast<uint32_t>(reader->sampleRate);
}

int32_t file_reader_read_samples(juce::AudioFormatReader *reader,
                                 float **buffers,
                                 int32_t num_channels,
                                 int64_t start_pos,
                                 int32_t num_samples) {
    return reader->read(buffers, num_channels, start_pos, num_samples) ? 1 : 0;
}

}