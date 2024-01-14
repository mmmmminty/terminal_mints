#!/bin/bash

BIN_NAME="merps"
DEST_DIR="/usr/local/bin"

cp "$BIN_NAME" "$DEST_DIR"

chmod +x "$DEST_DIR/$BIN_NAME"

echo "Installation complete. $BIN_NAME is now available in your PATH."
echo "Thanks for installing terminal mints! Type 'mints --help' to get started."
