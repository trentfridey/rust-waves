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

This code solves for $\psi$:

$$
\frac{\partial \psi(x,y,t)}{\partial t} = \frac{i\beta}{2}\nabla^2 \psi(x,y,t)
\qquad \text{on} \, \Omega \times T
$$
$$
\Omega = \begin{cases}
x \in [0,L]\\
y \in [0,L]\\
\end{cases}
$$
$$
T = [0, \infty)
$$

with homogeneous Dirichlet boundary conditions

$$
\psi|_{\partial \Omega} = 0  
$$
$$
\partial \Omega = \begin{cases}
x = 0 \\
x = L \\ 
y = 0 \\
y = L
\end{cases}
$$

and initial condition:

$$
\psi(x,y,0) = f(x,y)
$$

### Methods

It implements a finite difference algorithm with a centered difference in time for stability reasons (see [[1]](#1) for background).
We discretize on a spatial grid of spacing $\delta$, and in time by steps of size $\Delta t$: 

$$
\psi(j\delta, k\delta, n\Delta t) = \psi^n_{j,k}
$$

We approximately the derivatives thusly:

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
\beta \leq \frac{1}{2}\frac{\delta^2}{\Delta t} 
$$


The update rule in $\Omega$ is:

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

or, by inserting the stability criterion:

$$
\psi^{n+1}_{j,k} = 
  \psi^{n-1}_{j,k} - 
  iC
  \left[
    \psi^{n}_{j-\delta,k}+\psi^{n}_{j+\delta,k} +
    \psi^{n}_{j,k-\delta}+\psi^{n}_{j,k+\delta} -
    4\psi^{n}_{j,k}
  \right]
$$

where $C \leq \frac 12$

The boundary conditions imply:

$$
\left.\psi\right|_{\partial \Omega} = 0
\implies
\begin{cases}
\psi^{n}_{0,k} = 0 \\
\psi^{n}_{j,0} = 0 \\
\psi^{n}_{L,k} = 0 \\
\psi^{n}_{j,L} = 0 \\
\end{cases}
$$

and the initial condition implies:

$$
\psi^{1}_{j,k} = 
  \psi^{-1}_{j,k} - 
  iC
  \left[
    \psi^{0}_{j-\delta,k}+\psi^{0}_{j+\delta,k} +
    \psi^{0}_{j,k-\delta}+\psi^{0}_{j,k+\delta} -
    4\psi^{0}_{j,k}
  \right]
$$

Since $\psi^{-1}_{j,k}$ is undefined by the initial conditions, we need to solve for it in another way.

## References

<a id="1">[1]</a> *Numerical Methods and Causality in Physics*: https://arxiv.org/pdf/1302.5601.pdf