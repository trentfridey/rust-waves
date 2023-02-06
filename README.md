# Rust-Waves

A 2D simulator for the wave equation written in Rust and WebAssembly. 

Inspired by [jtiscione/webassembly-wave](https://github.com/jtiscione/webassembly-wave)

## Background:

This code solves for $\psi$:

$$
 \nabla^2 \psi(x,y,t) - \frac{1}{c^2}\frac{\partial^2 \psi(x,y,t)}{\partial t^2} = 0
\qquad \text{on} \, \Omega
$$
$$
\Omega = \begin{cases}
x \in [0,L]\\
y \in [0,L]\\
t \in [0, \infty)
\end{cases}
$$

with homogeneous Dirichlet boundary conditions

$$
\psi(x,y,t) = 0  \qquad x = 0, \, x = L, \, y = 0, \, y = L
$$

and initial condition:

$$
\psi(x,y,0) = f(x,y)
$$

It implements a finite difference algorithm by approximating second derivatives:

$$
\psi_{xx} \approx
  \psi^{n}_{j-1,k}
  -2\psi^{n}_{j,k}
  +\psi^{n}_{j+1,k}
$$


