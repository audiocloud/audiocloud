cmake_minimum_required(VERSION 3.24)

project(audio_engine_juce)

set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
set(CMAKE_OSX_DEPLOYMENT_TARGET "10.13" CACHE STRING "Support macOS down to High Sierra")

set(JUCE_MODULES_ONLY ON)
set(NEEDS_CURL OFF)
set(NEEDS_WEB_BROWSER OFF)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if (MSVC)
    set(CMAKE_CXX_FLAGS "/EHsc")
endif ()

add_subdirectory(JUCE)

add_library(
        audio_engine_juce
        STATIC
        ffi/juce.cpp
)

target_compile_definitions(
        audio_engine_juce
        PUBLIC
        JUCE_ASIO=1
)

target_link_libraries(
        audio_engine_juce
        PUBLIC
        juce::juce_audio_devices
        juce::juce_audio_basics
        juce::juce_audio_formats
)