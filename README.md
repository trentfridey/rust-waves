# Rust-Waves

A 2D simulator for the wave equation written in Rust and WebAssembly. 

Inspired by [jtiscione/webassembly-wave](https://github.com/jtiscione/webassembly-wave)

## Running Locally

1. npm run start

## TODOs:

- [x] Find way to live-reload on Rust changes (using [rust-parcel-template](https://github.com/rustwasm/rust-parcel-template))

Rust:
- [x] test `hsv_to_rgb` function and return types
- [x] plot colors for each `Complex<i32>` in unit disc (`z.norm() <= 1`)
- [ ] implement `step` function for `QWave`
  - [ ] implement normalization
  - [ ] implement reflections
- [ ] tests for `step`:
  - [x] test for $\delta$-function step
  - [ ] test for reflections
  - [ ] test for normalization

Front-end:
- [x] Load wasm for Schrodinger equation simulation
- [x] implement start / stop button
- [ ] implement intensity-only toggle
- [x] implement FPS counter

## Background:

The code implements a finite difference method.

For example, for the Schrodinger equation, the update rule comes from approximating the derivatives:

$$
i \frac{\partial \psi}{\partial t} \approx i( \psi(t+1,\vec{x}) - \psi(t,\vec{x}))
$$

$$
\nabla^2 \psi = \psi_{xx}(t,\vec{x}) + \psi_{yy}(t,\vec{x})
$$

$$
\psi_{xx} = u_{xx} + iv_{xx}
$$

$$
u_{xx} \approx \frac{u(x-h,y,t)-2u(x,y,t)+u(x+h,y,t)}{2}
$$

and likewise for the other coordinate and the imaginary part. Putting this altogether, the update rule is:

$$
u(t+1,\vec{x}) = u(t, \vec{x}) - (v_{xx}(t,\vec{x}) + v_{yy}(t,\vec{x}))
$$

$$
v(t+1,\vec{x}) = v(t, \vec{x}) - (u_{xx}(t,\vec{x}) + u_{yy}(t,\vec{x}))
$$

The stencil for the spatial derivative is:

$$
\nabla^2 \sim
\left[
\begin{array}{ccc}
0 & 1/2 & 0 \\
1/2 & -2 & 1/2 \\
0 & 1/2 & 0 \\
\end{array}
\right]
$$