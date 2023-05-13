cmake_minimum_required(VERSION 3.24)

project(audio_engine_juce)

set(JUCE_MODULES_ONLY ON)

set(CMAKE_OSX_DEPLOYMENT_TARGET "10.13" CACHE STRING "Support macOS down to High Sierra")
set(NEEDS_CURL OFF)
set(NEEDS_WEB_BROWSER OFF)

add_subdirectory(JUCE)

add_library(audio_engine_juce STATIC ffi/juce.cpp PUBLIC JUCE_ASIO=1)
target_compile_definitions(audio_engine_juce PUBLIC JUCE_ASIO=1)

target_link_libraries(
        audio_engine_juce
        PUBLIC
        juce::juce_audio_devices
        juce::juce_audio_basics
        juce::juce_audio_formats
)