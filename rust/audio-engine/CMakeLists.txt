cmake_minimum_required(VERSION 3.24)

project(audio_engine_juce)

set(CMAKE_OSX_DEPLOYMENT_TARGET "10.13" CACHE STRING "Support macOS down to High Sierra")

set(JUCE_MODULES_ONLY ON)
set(NEEDS_CURL OFF)
set(NEEDS_WEB_BROWSER OFF)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(JUCE)

add_library(
        audio_engine_juce
        STATIC
        ffi/juce.cpp
)

if (WIN32)
    include_directories(sdks/asio/asiosdk_2.3.3_2019-06-14/common)
endif ()

if (WIN32)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /EHsc")
    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /MDd /O2")
    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} /MDd /Zi /Od")
endif ()

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