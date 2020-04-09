mod utils;
extern crate num;

use num::complex::Complex;
use std::f32;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

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

#[no_mangle]
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

    fn psi_evolve(&self, row: u32, col: u32) -> Complex<f32> {
        // finite difference algorithm
        // computes psi_next a the point specified by row, col
        // using a spatial stencil:
        // [[0,1,0],[1,-4,1],[0,1,0]]
        // We also implement Neumann boundary conditions by modifiying the
        // stencil to enable reflections. For example, on the left wall the
        // stencil becomes: [[0,1,0],[0,-4,2],[0,1,0]]
        let mut psi_next = Complex { im: 0.0, re: 0.0 };
        let idx = self.get_index(row, col);
        let psi_ctr = self.psi[idx];
        let psi_neighbors = if col == 0 && row != 0 && row != self.height { 
            // left wall
                self.psi[self.get_index(row - 1, col)]
                + self.psi[self.get_index(row + 1, col)]
                + 2.0 * self.psi[self.get_index(row, col + 1)]
            } else if col == self.width && row != 0 && row != self.height {
                // right wall
                   self.psi[self.get_index(row - 1, col)]
                   + 2.0 * self.psi[self.get_index(row, col - 1)]
                   + self.psi[self.get_index(row + 1, col)]
            } else if row == 0 && col != 0 && col != self.width {
                // top wall
                    self.psi[self.get_index(row, col - 1)]
                    + 2.0 * self.psi[self.get_index(row + 1, col)]
                    + self.psi[self.get_index(row, col + 1)]
            } else if row == self.height && col != 0 && col != self.width {
                // bottom wall
                   2.0 * self.psi[self.get_index(row - 1, col)]
                   + self.psi[self.get_index(row, col - 1)]
                   + self.psi[self.get_index(row, col + 1)]
            } else {
                // default
                    self.psi[self.get_index(row - 1, col)]
                    + self.psi[self.get_index(row, col - 1)]
                    + self.psi[self.get_index(row + 1, col)]
                    + self.psi[self.get_index(row, col + 1)]
            };
        psi_next.re = -1.0 * (psi_neighbors.im - 4.0 * psi_ctr.im - psi_ctr.re);
        psi_next.im = psi_neighbors.re - 4.0 * psi_ctr.re - psi_ctr.im;
        return psi_next;
    }
}
#[wasm_bindgen]
impl Laboratory {
    #[no_mangle]
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

    #[no_mangle]
    pub fn step(&mut self) {
        let mut next = self.psi.clone();
        // normalize the result - if any cells have a non-zero value, count
        // let mut count = 0.0;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                next[idx] = self.psi_evolve(row, col);
                // if next[idx].norm() > 0.0 {
                //     count += 1.0
                // };
            }
        }
        let next_psi: Vec<Complex<f32>> = next.into_iter().map(|c| c).collect();
        self.image = next_psi.iter().map(|&c| complex_to_rgba(c)).collect();
        self.psi = next_psi;
    }

    #[no_mangle]
    pub fn image(&self) -> *const u32 {
        self.image.as_ptr()
    }
    #[no_mangle]
    pub fn width(&self) -> u32 {
        self.width
    }
    #[no_mangle]
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
