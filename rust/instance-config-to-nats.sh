#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export NATS_URL=${NATS_URL:-10.1.0.10:4222}

./target/debug/ac --nats-url=${NATS_URL} instance put ppdu_r1_up_l config/instances/ppdu_r1_up_l.yaml --host raspineve
./target/debug/ac --nats-url=${NATS_URL} instance put btrmkr_ml_1 config/instances/btrmkr_ml_1.yaml --host raspineve --mocked