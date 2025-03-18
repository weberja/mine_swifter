#!/bin/sh

echo "Remove old files"
sudo rm -r /usr/share/nginx/html/
set -e
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build
echo "Move Files"
sudo mkdir /usr/share/nginx/html
sudo cp -r ./dist/. /usr/share/nginx/html
echo "Set rights"
sudo chown --reference /usr/share/nginx/ -R /usr/share/nginx/html/
