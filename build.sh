#!/bin/sh

cargo build
wasm-pack build

cd pkg
npm link

cd ../www
npm install
npm link sassruist
npm run build
