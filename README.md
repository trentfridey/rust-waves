# Rust-Waves

A 2D simulator for the wave equation written in Rust and WebAssembly. 

Inspired by [jtiscione/webassembly-wave](https://github.com/jtiscione/webassembly-wave)

## Running Locally

1. npm run start

## TODOs:

- [x] Find way to live-reload on Rust changes (using [rust-parcel-template](https://github.com/rustwasm/rust-parcel-template))

Rust:
- [ ] test `hsv_to_rgb` function and return types
  - [ ] plot colors for each `Complex<i32>` in unit disc (`z.norm() <= 1`)
- [ ] implement `psi_evolve` function to compute next psi
  - [ ] implement normalization
  - [ ] implement reflections
- [ ] tests for `psi_evolve` function

Front-end:
- [ ] Load wasm for Schrodinger equation simulation
- [ ] implement start / stop button
- [ ] implement intensity-only toggle