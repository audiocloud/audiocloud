// TODO: currently this does nothing interesting
#include <memory>
#include <algorithm>
#include <juce_audio_devices/juce_audio_devices.h>
#include <juce_audio_formats/juce_audio_formats.h>

struct Mgr {
    std::unique_ptr <juce::AudioDeviceManager> device_manager;
    std::unique_ptr <juce::AudioIODevice> device;
};

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
Mgr *create_mgr(const char *type_name,
                const char *input_name, const char *output_name,
                const int input_channel_count, const int output_channel_count,
                const int sample_rate,
                const int buffer_size) {
    try {
        std::cout << "create_mgr: " << type_name << " " << input_name << " " << output_name << std::endl;
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

                return nullptr;
            }

            juce::BigInteger input_channels, output_channels;

            input_channels.setRange(0, input_channel_count, true);
            output_channels.setRange(0, output_channel_count, true);

            auto error = audio_device->open(input_channels, output_channels, sample_rate, buffer_size);
            if (!error.isEmpty()) {
                std::cerr << error << std::endl;
                return nullptr;
            }

            return new Mgr{
                    std::move(device_manager),
                    std::move(audio_device)
            };
        }

        std::cerr << "No device type found" << std::endl;
    } catch (...) {
        std::cerr << "create_mgr: exception" << std::endl;
    }

    return nullptr;
}

void delete_mgr(Mgr *mgr) {
    if (mgr->device->isOpen()) {
        mgr->device->close();
    }

    delete mgr;
}

void mgr_start(Mgr *mgr, void (*rust_callback)(
        void *data,
        const float *const *, int,
        float *const *, int,
        int), void *data) {

    if (mgr && mgr->device && rust_callback) {
        auto callback = std::make_unique<RustAudioIoCallback>();
        callback->rust_callback = rust_callback;
        callback->data = data;

        mgr->device->start(callback.release());
    }
}

uint32_t mgr_get_latency(Mgr *mgr) {
    if (mgr && mgr->device) {
        return mgr->device->getInputLatencyInSamples() + mgr->device->getOutputLatencyInSamples();
    }

    return 0;
}

void mgr_stop(Mgr *mgr) {
    if (mgr && mgr->device) {
        mgr->device->stop();
    }
}

static std::unique_ptr <juce::AudioFormatManager> format_manager;

void engine_init() {
    juce::initialiseJuce_GUI();
    format_manager = std::make_unique<juce::AudioFormatManager>();
    format_manager->registerBasicFormats();
}

void engine_shutdown() {
    format_manager.reset();
    juce::shutdownJuce_GUI();
}

}