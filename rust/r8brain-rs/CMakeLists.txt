cmake_minimum_required(VERSION 3.21)
set(CMAKE_OSX_DEPLOYMENT_TARGET "11.0" CACHE STRING "Minimum OS X deployment version")
project(r8brain)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")

include_directories(r8brain)
include_directories(r8brain/DLL)
include_directories(r8brain/pffft_double)
include_directories(r8brain/pffft_double/simd)

add_library(r8brain STATIC
        r8brain/pffft_double/simd/pf_avx_double.h
        r8brain/pffft_double/simd/pf_double.h
        r8brain/pffft_double/simd/pf_neon_double.h
        r8brain/pffft_double/simd/pf_neon_double_from_avx.h
        r8brain/pffft_double/simd/pf_scalar_double.h
        r8brain/pffft_double/simd/pf_sse2_double.h
        r8brain/pffft_double/pffft_double.c
        r8brain/pffft_double/pffft_double.h
        r8brain/pffft_double/pffft_priv_impl.h
        r8brain/pffft.cpp
        r8brain/pffft.h
        r8brain/DLL/r8bsrc.cpp
        r8brain/DLL/r8bsrc.h
        r8brain/CDSPBlockConvolver.h
        r8brain/CDSPFIRFilter.h
        r8brain/CDSPFracInterpolator.h
        r8brain/CDSPHBDownsampler.h
        r8brain/CDSPHBUpsampler.h
        r8brain/CDSPProcessor.h
        r8brain/CDSPRealFFT.h
        r8brain/CDSPResampler.h
        r8brain/CDSPSincFilterGen.h
        r8brain/fft4g.h
        r8brain/r8bbase.cpp
        r8brain/r8bbase.h
        r8brain/r8bconf.h
        r8brain/r8butil.h
        contrib/dll_additional.cpp)

target_compile_definitions(r8brain PUBLIC _cdecl= R8BSRC_DECL= R8B_PFFFT_DOUBLE)