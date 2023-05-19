#!/bin/bash

set -euxo pipefail

export NATS_URL=${NATS_URL:-10.1.0.10:4222}

# PATH=$PATH:./target/release/

./target/release/ac kv reset