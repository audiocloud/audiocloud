#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export NATS_URL=${NATS_URL:-10.1.0.10:4222}

./target/release/ac --nats-url="${NATS_URL}" user create 'rok' 'rok.kroflic@gmail.com'