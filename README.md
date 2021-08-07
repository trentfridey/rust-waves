# Rust-Waves

A 2D simulator for the wave equation written in Rust and WebAssembly. 

Inspired by [jtiscione/webassembly-wave](https://github.com/jtiscione/webassembly-wave)

## Running Locally

1. npm run start

## TODOs:

- [x] Find way to live-reload on Rust changes (using [rust-parcel-template](https://github.com/rustwasm/rust-parcel-template))

Rust:
- [x] implement Gaussian wave packet for `new` `QWave`
- [x] test `hsv_to_rgb` function and return types
- [x] plot colors for each `Complex<i32>` in unit disc (`z.norm() <= 1`)
- [ ] implement `step` function for `QWave`
  - [x] implement centered-time finite difference
  - [ ] debug integer overflow before hsv_to_rgba

Front-end:
- [x] Load wasm for Schrodinger equation simulation
- [x] implement start / stop button
- [x] implement intensity-only toggle
- [x] implement FPS counter
- [x] implement frame counter

## Background:

$$
\frac{\partial \psi}{\partial t} = \frac{i\hbar}{2m}\nabla^2 \psi
$$

The code implements a finite difference method; for the Schrodinger simulation, it uses the centered-difference in time for stability reasons (see [[1]](#1) for background and an surprising derivation of the energy-time uncertainty relation!). Explicitly this is:

$$
\frac{\partial \psi}{\partial t} \approx \frac{\psi^{n+1}_{j,k} - \psi^{n-1}_{j,k}}{2\Delta t}
$$

$$
\nabla^2 \psi = \psi_{xx}+ \psi_{yy}
$$

$$
\psi_{xx} \approx \frac{
  \psi^{n}_{j-\delta,k}
  -2\psi^{n}_{j,k}
  +\psi^{n}_{j+\delta,k}
  }{\delta^2}
$$


The stability criteria is:

$$
\frac{\hbar}{m} = \beta \leq \frac{1}{2}\frac{\delta^2}{\Delta t} 
$$


The update rule is:

$$
\psi^{n+1}_{j,k} = 
  \psi^{n-1}_{j,k} - 
  \frac{i\beta\Delta t}{\delta^2}
  \left[
    \psi^{n}_{j-\delta,k}+\psi^{n}_{j+\delta,k} +
    \psi^{n}_{j,k-\delta}+\psi^{n}_{j,k+\delta} -
    4\psi^{n}_{j,k}
  \right]
$$

## References

<a id="1">[1]</a> *Numerical Methods and Causality in Physics*: https://arxiv.org/pdf/1302.5601.pdf