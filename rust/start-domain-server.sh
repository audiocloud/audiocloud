#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export RUST_LOG=warn

export NATS_URL=10.1.0.10:4222
export REST_API_BIND=0.0.0.0:7200

./target/release/domain_server --nats-url=${NATS_URL} --enable-api --enable-instances --rest-api-bind=${REST_API_BIND}