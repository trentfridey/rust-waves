mod utils;
extern crate num;

use num::complex::Complex;
use std::f32;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

extern crate web_sys;
macro_rules! log {
     ($($t:tt)*) => {
         web_sys::console::log_1(&format!($($t)*).into());
     };
 }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Laboratory {
    width: u32,
    height: u32,
    psi: Vec<Complex<f32>>,
    image: Vec<u32>,
}

pub fn complex_to_rgba(complex: Complex<f32>) -> u32 {
    // takes psi at an index and computes
    // rgba value via norm -> value
    // and            phase -> hue
    // returns rgba value as a u32, i.e., 0xRRGGBBAA
    let value = complex.norm(); // range (0, 1)
    let hue = complex.arg(); // arg() uses atan2 to return angle in radians. range is (-PI, PI]
    let theta = if hue < 0.0 {
        hue * (180.0 / PI) + 360.0
    } else {
        (hue) * (180.0 / PI)
    };

    let red = {
        if 0.0 <= theta && theta <= 60.0 {
            value
        } else if 60.0 < theta && theta <= 120.0 {
            value * (120.0 - theta) / 60.0
        } else if 240.0 < theta && theta <= 300.0 {
            value * (theta - 240.0) / 60.0
        } else if 300.0 < theta && theta <= 360.0 {
            value
        } else {
            0.0
        }
    };

    let green = {
        if 0.0 < theta && theta <= 60.0 {
            value * (theta) / 60.0
        } else if 60.0 < theta && theta <= 180.0 {
            value
        } else if 180.0 < theta && theta <= 240.0 {
            value * (240.0 - theta) / 60.0
        } else {
            0.0
        }
    };

    let blue = {
        if 120.0 < theta && theta <= 180.0 {
            value * (theta - 120.0) / 60.0
        } else if 180.0 < theta && theta <= 300.0 {
            value
        } else if 300.0 < theta && theta <= 360.0 {
            value * (360.0 - theta) / 60.0
        } else {
            0.0
        }
    };

    let (r, g, b) = (red * 255.0, green * 255.0, blue * 255.0);
    return 0xff000000 | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32);
}

impl Laboratory {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn is_valid_index(&self, row: u32, col: u32) -> bool {
        (row < self.height) && (col < self.width)
    }

    fn psi_evolve(&self, row: u32, col: u32) -> Complex<f32> {
        // finite difference algorithm
        // computes psi_next a the point specified by row, col
        let mut psi_next = Complex { im: 0.0, re: 0.0 };
        let idx = self.get_index(row, col);
        // here we must implement the boundary conditions
        // if neighbor is out of bounds, it does not contribute to the center
        // yet a hard wall boundary means the spatial derivative must also vanish
        // which means the wave should reflect back
        let psi_ctr = self.psi[idx];
        let neighbors = if row != 0 && col != 0 {
            vec![
                (row - 1, col),
                (row, col - 1),
                (row + 1, col),
                (row, col + 1),
            ]
        } else if row != 0 && col == 0 {
            // wave has reached left wall, need to reflect to right
            vec![(row - 1, col), (row + 1, col), (row, col + 1)]
        } else if col != 0 && row == 0 {
            // wave has reached top wall, need to reflect down
            vec![(row, col - 1), (row + 1, col), (row, col + 1)]
        } else {
            // wave has reached top left corner, reflect down and right
            vec![(row + 1, col), (row, col + 1)]
        };
        let psi_neighbors: Complex<f32> = neighbors
            .into_iter()
            .map(|n| {
                if self.is_valid_index(n.0, n.1) {
                    self.psi[self.get_index(n.0, n.1)]
                } else {
                    Complex { re: 0.0, im: 0.0 }
                }
            })
            .fold(Complex { re: 0.0, im: 0.0 }, |acc, p| acc + p);
        psi_next.re = -1.0 * (psi_neighbors.im - 4.0 * psi_ctr.im - psi_ctr.re);
        psi_next.im = psi_neighbors.re - 4.0 * psi_ctr.re - psi_ctr.im;
        return psi_next;
    }
}

#[wasm_bindgen]
impl Laboratory {
    pub fn new() -> Laboratory {
        let height: u32 = 200;
        let width: u32 = 200;

        let psi_0: Vec<Complex<f32>> = (0..width * height)
            .map(|idx| {
                if idx >= 200 && idx < 400 {
                    Complex { re: 0.01, im: 0.0 }
                } else {
                    Complex { re: 0.0, im: 0.0 }
                }
            })
            .collect();
        let image_0 = psi_0
            .iter()
            .map(|&complex| complex_to_rgba(complex))
            .collect();
        Laboratory {
            height,
            width,
            psi: psi_0,
            image: image_0,
        }
    }

    pub fn step(&mut self) {
        let mut next = self.psi.clone();
        // normalize the result - if any cells have a non-zero value, count
        let mut count = 0.0;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                next[idx] = self.psi_evolve(row, col);
                if next[idx].norm() > 0.0 {
                    count += 1.0
                };
            }
        }
        let next_psi: Vec<Complex<f32>> = next.into_iter().map(|c| c).collect();
        self.image = next_psi.iter().map(|&c| complex_to_rgba(c)).collect();
        self.psi = next_psi;
    }

    pub fn image(&self) -> *const u32 {
        self.image.as_ptr()
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Laboratory {
    pub fn get_psi(&self) -> &[Complex<f32>] {
        &self.psi
    }

    pub fn set_psi(&mut self, psi: &[(u32, u32, Complex<f32>)]) {
        for point in psi.iter() {
            let idx = self.get_index(point.0, point.1);
            self.psi[idx] = point.2
        }
    }

    pub fn delta_psi(&mut self, point: &(u32, u32)) {
        let idx = self.get_index(point.0, point.1);
        self.psi[idx] = Complex { re: 1.0, im: 0.0 }
    }
}
