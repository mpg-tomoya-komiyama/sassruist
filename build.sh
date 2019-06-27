#!/bin/sh

wasm-pack build
cd ./www
rm -rf dist/
npm install
npm run build
