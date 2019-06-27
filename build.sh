#!/bin/sh

cargo build
wasm-pack build

cd ./www
rm -rf dist/
npm install
npm run build
