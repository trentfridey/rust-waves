# Rust-Waves

A 2D simulator for the wave equation written in Rust and WebAssembly. 

Inspired by [jtiscione/webassembly-wave](https://github.com/jtiscione/webassembly-wave)

## Running Locally

1. Build with `wasm-pack build`
2. `cd www/`
3. `npm start`

## TODOs:

Rust:
- [ ] test `hsv_to_rgb` function and return types
- [ ] implement `psi_evolve` function to compute next psi
  - [ ] implement normalization
  - [ ] implement reflections
- [ ] tests for `psi_evolve` function

Front-end:
- [ ] Load wasm for Schrodinger equation simulation
- [ ] implement start / stop button
- [ ] implement intensity-only toggle