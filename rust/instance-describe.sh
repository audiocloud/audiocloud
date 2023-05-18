#!/bin/bash

set -euxo pipefail

export NATS_URL=${NATS_URL:-10.1.0.10:4222}

PATH=$PATH:./target/release

ac instance describe ppdu_r1_up_l
ac instance describe btrmkr_ml_1
ac instance describe tierra_gravity_1