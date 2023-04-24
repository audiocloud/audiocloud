project(audio_engine_juce)

cmake_minimum_required(VERSION 3.24)

set(JUCE_MODULES_ONLY ON)

set(CMAKE_OSX_DEPLOYMENT_TARGET "10.13" CACHE STRING "Support macOS down to High Sierra")

add_subdirectory(JUCE)

add_library(audio_engine_juce STATIC ffi/juce.cpp)

target_link_libraries(
        audio_engine_juce
        PUBLIC
        juce::juce_audio_devices
        juce::juce_audio_basics
        juce::juce_audio_formats
)