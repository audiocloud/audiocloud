# About

Audio Cloud is a collection of services that schedule, execute and automate software supported tasks dealing with
multimedia content creation and manipulation. Many systems exist that already execute e.g. media format conversion
tasks, image generation tasks, video subtitling and similar operations. Audio Cloud is different in a few key areas:

- **Interactivity**: Users can connect to and manipulate tasks in real time with a client application
- **Mixed processing**: Tasks execute on computers using software algorithms, on specialized hardware or a mix of both
- **Scalability**: Can scale to a large number of tasks and users
- **Security**: Designed with security in mind and to protect the privacy of users, their data and the data of other
  users
- **Reliability**: Reliable and able to recover from failures quickly
- **Performance**: Low oveerhead, low latency near real-time execution of tasks in a timely manner
- **Cost**: Cost effective by running on commodity hardware and utilizing open source software
- **Interoperability**: The system is designed to be interoperable and to be able to work with other systems, algorithms
  and hardware

## Concepts and architecture

An audio cloud system consists of at least one domain and at least one audio engine. A domain is a self sufficient
collection of hardware and softaware that can execute tasks. It implements task level security, scheduling, media
retrieval, hardware lifecycle and realtime interaction with end user applications. It does not implement cross-domain
security, orchestration or resource limiting.

A task is defined as a graph of processing nodes called **instances**, connected in a directed graph. Each instance is
either allocated dynamically from the software resources available on the audio engine (such as CPU, memory, GPU, etc.)
or is allocated from a pool of hardware instances attached directly to an audio engine. Software defined instances are
called **dynamic** while the hardware instances are called **fixed**.

Each instance regardless of type (dynamic or fixed) has an associated **model**. This specifies inputs, outputs,
parameters that can be set on the instance and reports that it generates while in use.

An audio cloud system could also include an orchestrator.

# Reference implementation

The `api` repository contains shared type definitions and APIs while other repositories contain reference
implementations of all the services. In principle, any reference service could be replaced with an alternate
implementation adhering to the same API and the system would work.

The reference audio engine and domain service in particular are services that are expected to be replaced or augmented
by other implementations.

## REAPER Audio Engine Service

The reference audio engine service uses the [REAPER](https://reaper.fm) digital audio workstation as the underlying
implementation and is subject to licensing requirements - e.g. you will need a valid REAPER license to use it, even if
it is not for production use. Furthermore, REAPER source is not available and the service plugs into REAPER through the
built-in VST plugin interface.

## Domain Service

The reference domain service contains specifics that make it easy for Distopik Ltd., staff to deploy and manage a fleet
of devices for the [mixanalog.com](https://mixanalog.com) service. It is not intended as a one size fits all / general
purpose domain service implementation, but a starting point for further customization.

## Runtime

Audio Cloud Reference service implemented with help of [Actix](https://actix.rs/), a rust actor framework and actix-web,
the companion high performance web framework for actix.

# System patterns

Audio Cloud is designed as a collaborative system of services that contain agents (actors). Each resource is modeled as
inert data in a database when not in use and as an actor executing logic and timers when in use. The system passes a
specification of a desired outcome from APIs to the actors, which then collaborate to arrive at this desired outcome.
During this time, resources or subtasks may fail, be busy or be replaced.

# Technology choice

[Rust](https://www.rust-lang.org/) was chosen as the implementation language. Multimedia tasks are generally implemented
in C/C++ owing to low overhead and direct access to hardware resources and operating system APIs. Rust is a systems
programming language that inherits these qualities from C++ but combines them with a modern type system without
inheritance, memory safety and a rich, lively ecosystem of libraries.

There are plenty of high quality Libraries in Rust for writing distributed, high performance and multi-threaded services
but the multimedia support is not as mature. The Audio Cloud project will have to fill in the blanks and contribute back
to the open source community in that area.
