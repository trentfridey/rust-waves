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

const ALPHA: u32 = 0xFF_00_00_00; 

#[derive(Clone)]
enum Status {
    Default,
    Wall
}

#[wasm_bindgen]
pub struct Laboratory {
    width: u32,
    height: u32,
    psi: Vec<Complex<i32>>,
    image: Vec<u32>,
    status: Vec<Status>,
    t: u64
}

trait HexColor {
    fn to_rgb(self) -> u32;
}

impl HexColor for Complex<i32> {

}

#[no_mangle]
pub fn complex_to_rgba(complex: Complex<i32>, intensity_only: bool) -> u32 {
    // maps complex float to HSV 
    // if intensityOnly is true, then just map norm squared to HSV
    // rgba value via norm -> value
    // and            phase -> hue
    // returns rgba value as a u32, i.e., 0xRRGGBBAA
    // TODO: add toggle for intensity_only to be set on front end
    // TODO: why does this look blue when it renders?
    let value = complex.re.abs(); 
    return 0xff000000 | (value as u32) << 0x10 | (value as u32) << 0x08 | (value as u32)
    // TODO: refactor below to use fixed-point
    // let hue = complex.arg(); // arg() uses atan2 to return angle in radians. range is (-PI, PI]
    // let theta = if hue < 0.0 {
    //     hue * (180.0 / PI) + 360.0
    // } else {
    //     (hue) * (180.0 / PI)
    // };

    // let red = {
    //     if 0.0 <= theta && theta <= 60.0 {
    //         value
    //     } else if 60.0 < theta && theta <= 120.0 {
    //         value * (120.0 - theta) / 60.0
    //     } else if 240.0 < theta && theta <= 300.0 {
    //         value * (theta - 240.0) / 60.0
    //     } else if 300.0 < theta && theta <= 360.0 {
    //         value
    //     } else {
    //         0.0
    //     }
    // };

    // let green = {
    //     if 0.0 < theta && theta <= 60.0 {
    //         value * (theta) / 60.0
    //     } else if 60.0 < theta && theta <= 180.0 {
    //         value
    //     } else if 180.0 < theta && theta <= 240.0 {
    //         value * (240.0 - theta) / 60.0
    //     } else {
    //         0.0
    //     }
    // };

    // let blue = {
    //     if 120.0 < theta && theta <= 180.0 {
    //         value * (theta - 120.0) / 60.0
    //     } else if 180.0 < theta && theta <= 300.0 {
    //         value
    //     } else if 300.0 < theta && theta <= 360.0 {
    //         value * (360.0 - theta) / 60.0
    //     } else {
    //         0.0
    //     }
    // };

    // let (r, g, b) = (red * 255.0, green * 255.0, blue * 255.0);
    // return 0xff000000 | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32);
}

impl Laboratory {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
    fn clamp(&self, psi: Complex<i32>) -> Complex<i32> {
        if psi.re >= 255 {
            if psi.im >= 255 {
                return Complex{re: 255, im: 255}
            }
            return Complex{re: 255, im: psi.im}
        }
        if psi.im > 255 {
            return Complex{ re: psi.re, im: 255}
        }
        return Complex{ re: psi.re, im: psi.im }
    }
    fn psi_evolve(&self, row: u32, col: u32) -> Complex<i32> {
        // finite difference algorithm
        // computes psi_next a the point specified by row, col
        // using a spatial stencil:
        // [[0,1,0],[1,-4,1],[0,1,0]]
        // We also implement Neumann boundary conditions by modifiying the
        // stencil to enable reflections. For example, on the left wall the
        // stencil becomes: [[0,1,0],[0,-4,2],[0,1,0]]
        let left =  0;
        let right = self.width - 1;
        let top = 0;
        let bottom = self.height - 1;

        let empty: Complex<i32> = Complex{ re: 0, im: 0};
        let left_neighbor: Complex<i32> = if col > left { self.psi[self.get_index(row, col - 1)]} else { empty };
        let right_neighbor: Complex<i32> = if col < right { self.psi[self.get_index(row, col + 1)] } else { empty };
        let top_neighbor: Complex<i32> = if row > top { self.psi[self.get_index(row - 1, col)] } else { empty };
        let bottom_neighbor: Complex<i32> = if row < bottom { self.psi[self.get_index(row + 1, col)] } else { empty };
        let default_neighbors: Complex<i32> = left_neighbor + right_neighbor + top_neighbor + bottom_neighbor;
        // TODO: debug reflections
        let psi_neighbors: Complex<i32> = match (row, col) {
            (0, 0) => default_neighbors +  (bottom_neighbor + right_neighbor) / 2,
            (0, col) if col == right => default_neighbors + (bottom_neighbor + left_neighbor) / 2,
            (0, _) => default_neighbors + bottom_neighbor,
            (row, 0) if row == bottom => default_neighbors +  (top_neighbor + right_neighbor) / 2,
            (row, col) if row == bottom && col == right => default_neighbors +  (top_neighbor + left_neighbor) / 2,
            (row, _) if row == bottom => default_neighbors + top_neighbor,
            (_, 0) => default_neighbors + right_neighbor,
            (_, col) if col == right => default_neighbors + left_neighbor,
            (_, _) => default_neighbors
        };

        let mut psi_next = Complex{ re: 0, im: 0};
        let idx = self.get_index(row, col);
        let psi_ctr = self.psi[idx];
        // TODO: debug numerical errors causing instabilities
        
        psi_next.re = -1 * (psi_neighbors.im - 4 * psi_ctr.im - psi_ctr.re);
        psi_next.im = psi_neighbors.re - 4 * psi_ctr.re - psi_ctr.im;
        return self.clamp(psi_next);
    }
}
#[wasm_bindgen]
impl Laboratory {
    #[no_mangle]
    pub fn new() -> Laboratory {
        console_error_panic_hook::set_once();

        let height: u32 = 200;
        let width: u32 = 200;

        let psi_0: Vec<Complex<i32>> = (0..width * height)
            .map(|idx| {
                let x: i32 = (idx % width) as i32;
                let y: i32 = (idx / width) as i32;
                if x == 100 && y == 100  {
                   return Complex { re: 1, im: 0}; 
                }
                return Complex { re: 0, im: 0 };
            })
            .collect();
        let image_0 = psi_0
            .iter()
            .map(|&complex| complex_to_rgba(complex, false))
            .collect();
        Laboratory {
            height,
            width,
            psi: psi_0,
            image: image_0,
            t: 0
        }
    }

    #[no_mangle]
    pub fn step(&mut self) {
        let mut next = self.psi.clone();
        // TODO: normalize the result - if any cells have a non-zero value, count
        let mut count: i32 = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                next[idx] = self.clamp(self.psi_evolve(row, col));
                if next[idx].re > 0 || next[idx].im > 0 { count += 1 }
            }
        }
        // let next_psi = next.iter().collect();
        let next_psi: Vec<Complex<i32>> = next.into_iter().map(|p| Complex{re: p.re/count, im: p.im/count}).collect::<Vec<Complex<i32>>>();
        self.image = next_psi.iter().map(|&c| complex_to_rgba(c, false)).collect();
        self.psi = next_psi;
    }

    // TODO: remove float-based code
    // #[no_mangle]
    // pub fn exact_step(&mut self) {
    //     let a: f32 = 0.01;
    //     let t = self.t as f32;
    //     let next_psi: Vec<Complex<f32>> = (0..self.width*self.height)
    //     .map(|idx|{
    //         let x: f32 = (idx % self.width) as f32 - ((self.width / 2) as f32);
    //         let y: f32 = (idx / self.width) as f32 - ((self.height / 2) as f32);
    //         let w: f32 = a*x*x + a*y*y;
    //         let s: f32 = 1.0 + 4.0*t*t*a*a;
    //         return Complex { re: 1.0/s * (-w/s).exp() * (2.0*t*a*w/s).cos(), im: (1.0/s * (-w/s).exp() * (2.0*t*a*w/s).sin()) };
    //     })
    //     .collect();
    //     self.t = (t + 1.0) as u64;
    //     self.image = next_psi.iter().map(|&c| complex_to_rgba(c, false)).collect();
    //     self.psi = next_psi;
    // }

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
    pub fn get_psi(&self) -> &[Complex<i32>] {
        &self.psi
    }

    pub fn set_psi(&mut self, psi: &[(u32, u32, Complex<i32>)]) {
        for point in psi.iter() {
            let idx = self.get_index(point.0, point.1);
            self.psi[idx] = point.2
        }
    }

    pub fn delta_psi(&mut self, point: &(u32, u32)) {
        let idx = self.get_index(point.0, point.1);
        self.psi[idx] = Complex { re: 1, im: 0 }
    }
}
