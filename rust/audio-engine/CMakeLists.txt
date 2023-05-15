cmake_minimum_required(VERSION 3.24)

project(audio_engine_juce)

set(CMAKE_OSX_DEPLOYMENT_TARGET "11.0" CACHE STRING "Minimum OS X deployment version")
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")

set(JUCE_MODULES_ONLY ON)
set(NEEDS_CURL OFF)
set(NEEDS_WEB_BROWSER OFF)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if (WIN32)
    include_directories(sdks/asio/asiosdk_2.3.3_2019-06-14/common)

    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /EHsc")
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