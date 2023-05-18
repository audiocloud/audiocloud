#!/bin/bash

set -euxo pipefail

export INSTANCE_HOST=${INSTANCE_HOST:-raspineve}
export NATS_URL=${NATS_URL:-10.1.0.10:4222}

PATH=$PATH:./target/release

ac instance put ppdu_r1_up_l config/instances/ppdu_r1_up_l.yaml --host "${INSTANCE_HOST}"
ac instance put btrmkr_ml_1 config/instances/bettermaker/btrmkr_ml_1.yaml --host "${INSTANCE_HOST}"
ac instance put tierra_gravity_1 config/instances/tierra/tierra_gravity_1.yaml --host "${INSTANCE_HOST}"
ac instance put distopik_1176_1 config/instances/distopik/distopik_1176_1.yaml --host "${INSTANCE_HOST}"
ac instance put gyraf_g24_1 config/instances/gyraf/gyraf_g24_1.yaml --host "${INSTANCE_HOST}"
