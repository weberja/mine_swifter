#!/bin/sh
set -e

RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build
sudo docker stop game_server
sudo docker rm game_server
sudo docker buildx build -t mineswifter .
sudo docker run --name game_server -d -p 8081:80 mineswifter
