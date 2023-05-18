#!/bin/bash

set -euxo pipefail

TARGET_DIR="/Users/distopik2/audiocloud/target/release"

# Get the latest release information
RELEASE_INFO=$(curl -s "https://api.github.com/repos/audiocloud/audiocloud/releases/latest")

# Find the assets with 'macos' in the name
ASSET_URLS=$(echo "$RELEASE_INFO" | jq -r '.assets[] | select(.name | contains("macos")) | .browser_download_url')

if [[ -n $ASSET_URLS ]]; then
  IFS=$'\n'
  for DOWNLOAD_URL in $ASSET_URLS; do
    # Extract the asset name from the URL
    ASSET_NAME=$(basename "$DOWNLOAD_URL")

    # Rename the file if it starts with 'ac-'
    if [[ $ASSET_NAME == ac-* ]]; then
      RENAMED_ASSET_NAME="ac"
    # Rename the file if it starts with 'domain-server-'
    elif [[ $ASSET_NAME == domain-server-* ]]; then
      RENAMED_ASSET_NAME="domain_server"
    else
      RENAMED_ASSET_NAME="$ASSET_NAME"
    fi

    # Download the release asset
    echo "Downloading $ASSET_NAME..."
    curl -L -O "$DOWNLOAD_URL"

    # Make the downloaded file executable
    chmod +x "$ASSET_NAME"

    # Move the downloaded content to the target directory with the renamed name
    mkdir -p "$TARGET_DIR"
    mv "$ASSET_NAME" "$TARGET_DIR/$RENAMED_ASSET_NAME"

    echo "Download complete!"
  done
else
  echo "No 'macos' assets found. Skipping download."
fi