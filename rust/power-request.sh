#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export NATS_URL=${NATS_URL:-10.1.0.10:4222}

PATH=$PATH:./target/release

ac --nats-url=${NATS_URL} instance power btrmkr_ml_1 on -d 36000
ac --nats-url=${NATS_URL} instance power tierra_gravity_1 on -d 36000