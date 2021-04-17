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
    fn to_rgba(self) -> u32;
}

impl HexColor for Complex<i32> {
    // This should map the amplitude and phase represented by the Complex<i32>
    // to a color.
    // The most straightforward way would map the phase to the hue, so we use HSV, and then convert to RGB.
    // Since HSV is 3D, but the Complex<i32> is two dimensional, we set the saturation value to 1
    // This constrains the space to the surface of a cone: the radial angle is the hue, and the height
    // is the value, with the point of the cone corresponding to black.
    fn to_rgba(self) -> u32 {
        let hue = self.arg(); // should this cast an i32 to a float?
        let value = self.norm(); // should this also cast to a float?
        let (r,g,b) = hsv_to_rgb(hue, 1, value);
        return ALPHA | r << 16 | g << 8 | b
    } 
}

pub fn hsv_to_rgb((hue, sat, val): (u32, u32, u32)) -> (u8, u8, u8) {
    // TODO: test return types
    // based on algorithm from https://en.wikipedia.org/wiki/HSL_and_HSV
    // assume H in [0,360], S in [0,1], V in [0,1]
    let chroma: u8 = sat * val;  // chance of overflow?
    let hue_div: u32 = hue / 60; // this should return a float in [0,6] 
                                 // for matching
    const x: u8 = chroma * (1-((hue_div % 2) - 1).abs()); // this should be u8
    let (r1, g1, b1) = match hue_div {
        h if h < 1 => (chroma, x, 0),
        h if h < 2 => (x, chroma, 0),
        h if h < 3 => (0, chroma, x),
        h if h < 4 => (0, x, chroma),
        h if h < 5 => (x, 0, chroma),
        h if h < 6 => (chroma, 0, x),
        _ => (0,0,0)
    };
    const m: u32 = val - chroma;
    let (r,g,b) = (r1+m, g1+m, b1+m);
    return (r,g,b);
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

        let width: u32 = 200;
        let height: u32 = 200;

        let w: usize = width as usize;
        let h: usize = height as usize;

        let mut status = vec![Status::Default; w * h];
        let mut psi_0: Vec<Complex<i32>> = vec![0; w * h];

        for i in 0..h {
            status[i * w] = Status::Wall;
            status[i * w + w - 1] = Status::Wall;
        }
        // top and bottom walls
        for j in 0..w {
            status[j] = Status::Wall;
            status[w * h - w + j] = Status::Wall;
        }

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
        let w = self.width as usize;
        let h = self.height as usize;
        for row in 0..w * h {
            match self.status[i] {
                Status::Default => {
                    // TODO: implement psi-evolve
                },
                _ => {}
            }
        }
        let next_psi: Vec<Complex<i32>> = next.into_iter().map(|p| Complex{re: p.re/count, im: p.im/count}).collect::<Vec<Complex<i32>>>();
        self.image = next_psi.iter().map(|&c| c.to_rgba()).collect();
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
