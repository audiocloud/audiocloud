#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export NATS_URL=${NATS_URL:-10.1.0.10:4222}

cargo run --bin domain_server -- --enable-media --enable-instance-drivers --enable-instances demo.audiocloud.io