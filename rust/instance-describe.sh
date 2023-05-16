#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export NATS_URL=${NATS_URL:-10.1.0.10:4222}

./target/release/ac --nats-url="${NATS_URL}" instance describe ppdu_r1_up_l
./target/release/ac --nats-url="${NATS_URL}" instance describe btrmkr_ml_1
./target/release/ac --nats-url="${NATS_URL}" instance describe tierra_gravity_1