#!/usr/bin/bash
cd ~/trent/dev/rust-waves &&
wasm-pack build &&
cd www &&
npm start