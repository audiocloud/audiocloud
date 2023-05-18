#!/bin/bash

set -euxo pipefail

NATS_URL=${NATS_URL:-10.1.0.10:4222}

./target/release/ac --nats-url="${NATS_URL}" user create 'rok' 'rok.kroflic@gmail.com'