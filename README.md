# Rust-waves

A experimental project with Rust and WebAssembly to simulate the 2D wave equation using a finite-difference method.

## Theory

The project simulates the time evolution of a quantum particle in a 2D box. 
That is, it is the solution to the Schrodinger equation:

$$
i \frac{\partial \psi}{\partial t} = -\frac{1}{2} \nabla^2 \psi + V\psi
$$

Where

$$
V = \begin{cases}
    0, && 0 \leq x, y < L \\
    \infty, && |x|,|y| \geq L
\end{cases}
$$

This means that the walls are perfectly reflecting and so we impose *Neumann* boundary conditions:

$$
\hat{n}\cdot\nabla\psi\big|_{\partial \Omega} = 0
$$

## Methods

The code implements a centered difference scheme in space, and a forward difference scheme in time:

$$
\nabla^2 \psi(x,y,t) \approx \psi(x-1,y,t) + \psi(x,y-1,t) - 4\psi(x,y,t) + \psi(x+1,y,t) + \psi(x,y+1,t)\\
\psi_t(x,y,t) \approx \psi(x,y,t+1) - \psi(x,y,t)
$$

For the boundary conditions, the code implements reflections using a centered difference approximation of the first derivative. 
So for example:

$$
\left.\frac{\partial\psi}{\partial x}\right|_{x=0} \approx \frac{\psi(-1,y,t) - \psi(1,y,t)}{2} = 0 
$$

Which is used to modify the implementation of the spatial derivative.