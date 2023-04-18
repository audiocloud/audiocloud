#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export NATS_URL=${NATS_URL:-10.1.0.10:4222}

if [ -z "$1" ]
then
  echo "Usage: $0 <media_id>"
  exit 1
fi


cargo run --bin ac -- media download "$1" create https://www.soundhelix.com/examples/mp3/SoundHelix-Song-7.mp3 b7da992345804fd3d14a4cca943f51339082cad0414e71a4fea7187ce8c2e3f6 10095020