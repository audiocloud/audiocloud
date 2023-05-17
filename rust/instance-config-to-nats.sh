#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export NATS_URL=${NATS_URL:-10.1.0.10:4222}

./target/release/ac --nats-url=${NATS_URL} instance put ppdu_r1_up_l config/instances/ppdu_r1_up_l.yaml --host raspineve

./target/release/ac --nats-url=${NATS_URL} instance put btrmkr_ml_1 config/instances/bettermaker/btrmkr_ml_1.yaml --host raspineve
./target/release/ac --nats-url=${NATS_URL} instance put tierra_gravity_1 config/instances/tierra/tierra_gravity_1.yaml --host raspineve
./target/release/ac --nats-url=${NATS_URL} instance put distopik_1176_1 config/instances/distopik/distopik_1176_1.yaml --host raspineve
./target/release/ac --nats-url=${NATS_URL} instance put gyraf_g24_1 config/instances/gyraf/gyraf_g24_1.yaml --host raspineve