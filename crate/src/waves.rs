extern crate num;
use std::f32::consts::PI;

use num::complex::Complex;
use utils::{Status, Arena, Rectified, from_amp, to_amp};
use colors::{HexColor};

pub const FORCE_DAMPING_BIT_SHIFT: u8 = 4;
const STABILITY_PARAM: u8 = 6;

pub struct CWave {
    u: Vec<i32>,
    v: Vec<i32>,
    force: Vec<i32>,
}
#[derive(Debug)]
pub struct QWave {
    psi: Vec<Complex<i32>>,
    psi_prev: Vec<Complex<i32>>,
    pub norm: f32
}

pub trait Waveable {
    fn new(arena: &Arena) -> Self;
    fn step(&mut self, arena: &Arena, damping: u8);
    fn render(&self, norm_only: bool) -> Vec<u32>;
}

impl Waveable for CWave {
    fn new(arena: &Arena) -> CWave {
        let w: usize = arena.width as usize;
        let h: usize = arena.height as usize;
        let mut u_0: Vec<i32> = vec![0; w * h];
        for i in 0..w {
            for j in 0..h {
                if i > 50 && j > 50 && i < 100 && j < 100 {
                    u_0[j * w + i] = 0x3fffffff
                }
            }
        }
        let v_0: Vec<i32> = vec![0; w * h];
        let force_0: Vec<i32> = vec![0; w * h];
        return CWave {
            u: u_0,
            v: v_0,
            force: force_0,
        }
    }
    fn step(&mut self, arena: &Arena, damping: u8) {
        let w = arena.width as usize;
        let h = arena.height as usize;
        for i in 0..w * h {
            match arena.status[i] {
                Status::Default => {
                    let u_cen = self.u[i];
                    let u_north = self.u[i - w];
                    let u_south = self.u[i + w];
                    let u_east = self.u[i + 1];
                    let u_west = self.u[i - 1];
                    let uxx = ((u_west + u_east) >> 1) - u_cen;
                    let uyy = ((u_north + u_south) >> 1) - u_cen;
                    let mut vel = self.v[i] + (uxx >> 1) + (uyy >> 1);
                    if damping > 0 {
                        vel -= vel >> damping;
                    }
                    self.v[i] = vel.apply_cap();
                },
                _ => {}
            }
        }
        // Apply velocity and forces
        for i in 0..w * h {
            match arena.status[i] {
                Status::Default => {
                    let mut f = self.force[i];
                    self.u[i] = (f + (self.u[i] + self.v[i]).apply_cap()).apply_cap();
                    f -= f >> FORCE_DAMPING_BIT_SHIFT;
                    self.force[i] = f;
                },
                _ => {}
            }
        }
    } 
    fn render(&self, _norm_only: bool) -> Vec<u32> {
        return self.u.iter().map(|u: &i32| u.to_rgba(false)).collect();
    }
}

impl Waveable for QWave {
    fn new(arena: &Arena) -> QWave {
        let w: usize = arena.width as usize;
        let h: usize = arena.height as usize;

        let gauss = |x: i32, y: i32| -> f32 {
            let xf = x as f32 / arena.width as f32; 
            let yf = y as f32 / arena.height as f32;
            (-xf * xf - yf * yf).exp()
        };

        let wave_packet = |x: i32, y: i32| -> Complex<f32> {
            let xf = x as f32 / arena.width as f32;
            let yf = x as f32 / arena.height as f32;
            return Complex { re: (16.0*xf).cos()*gauss(16*x, 10*y), im: (16.0*xf).sin()*gauss(16*x, 10*y) }
        };
        
        // compute ψ
        let init_psi = |x, y| -> Complex<f32> {
             return wave_packet(x,y)
        };

        let mut psi_0: Vec<Complex<i32>> = vec![Complex{re: 0, im: 0}; w*h];
        let n = (0..w*h).into_iter().fold(0.0, |acc, i| { 
            let (x,y) = arena.to_xy(i);
            let amp = init_psi(x,y);
            acc + (amp * amp)
         });

        for i in 0..w * h {
            let (x,y) = arena.to_xy(i);
            let amp = init_psi(x,y) / n;
            let amp_i32 = (2 << 16)*from_amp(amp);
            psi_0[i] = Complex{ re: amp_i32, im: 0 };
        }
        let psi_prev = psi_0.clone();
        return QWave {
            psi: psi_0,
            psi_prev: psi_prev,
            norm: n
        }
    }
    fn step(&mut self, arena: &Arena, _damping: u8) {

        // finite difference algorithm

        let w = arena.width as usize;
        let h = arena.height as usize;

        let mut u_cen = self.psi.clone();
        let mut u_west: Complex<i32>;
        let mut u_east: Complex<i32>;
        let mut u_north: Complex<i32>;
        let mut u_south: Complex<i32>;
        let mut uxx = vec![Complex{re: 0, im: 0}; w * h];
        let mut uyy = vec![Complex{re: 0, im: 0}; w * h];

        for i in 0..w * h {
                match arena.status[i] {
                    Status::Default => {
                        u_cen[i] = self.psi[i];
                        u_west = self.psi[i - 1];
                        u_east = self.psi[i + 1];
                        u_north = self.psi[i - w];
                        u_south = self.psi[i + w];
                        uxx[i] = Complex{ re: u_west.re + u_east.re, im: u_west.im + u_east.im} - u_cen[i];
                        uyy[i] = Complex{ re: u_south.re + u_north.re, im: u_south.im + u_north.im} - u_cen[i];
                    },
                    _ => {}
            }
        }
        for i in 0..w * h {
            match arena.status[i] {
                Status::Default => {
                    // set n -> n+1
                    self.psi[i].re = self.psi_prev[i].re - (uxx[i].im + uyy[i].im) >> 3;
                    self.psi[i].im = self.psi_prev[i].im - (uxx[i].re + uyy[i].re) >> 3;
                    // set n-1 -> n
                    self.psi_prev[i] = u_cen[i]
                },
                _ => {}
            }
        }
        self.norm = self.psi
            .iter()
            .fold(0.0, |acc, x| { 
                let n = to_amp(x.re).powi(2) + to_amp(x.im).powi(2);
                acc + n
            });
    }
    fn render(&self, norm_only: bool) -> Vec<u32> {
        return self.psi.iter().map(|x| x.to_rgba(norm_only)).collect();
    }
}
