mod utils;
extern crate num;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const ALPHA: u32 = 0xFF000000;

const DEFAULT: u8 = 0;
const WALL: u8 = 1;
const PTRANSMITTER: u8 = 2;
const NTRANSMITTER: u8 = 3;
const FORCE_DAMPING_BIT_SHIFT: u8 = 4;

#[wasm_bindgen]
pub struct Arena {
    width: u32,
    height: u32,
    u: Vec<i32>,
    v: Vec<i32>,
    force: Vec<i32>,
    image: Vec<u32>,
    status: Vec<u8>,
}

pub fn applyCap(x: i32) -> i32 {
    if x < i32::MIN >> 1 {
        return i32::MIN >> 1;
    } else if x > i32::MAX >> 1 {
        return i32::MAX >> 1;
    } else {
        return x;
    }
}

pub fn toRGB(x: i32) -> u32 {
    let mut val: i32 = x >> 22;
    if val > 0 {
        let res = val as u32;
        return ((res << 8) | (res << 16) | ALPHA);
    } else {
        val = std::cmp::max(val, -255i32);
        return (-1i32 * val) as u32 | ALPHA;
    }
}

#[wasm_bindgen]
impl Arena {
    #[no_mangle]
    pub fn new() -> Arena {
        console_error_panic_hook::set_once();
        let width: u32 = 200;
        let height: u32 = 200;
        let w: usize = width as usize;
        let h: usize = height as usize;
        let mut status = vec![DEFAULT; w * h]; 
        let mut u_0: Vec<i32> = vec![0; w*h]; 
        // Draw walls along the outer boundary
        for i in 0..h {
            status[i * w] = WALL;
            status[i * w + w - 1] = WALL;
        }
        for j in 0..w {
            status[j] = WALL;
            status[w * h - w + j] = WALL;
        }

        for i in 0..w {
            for j in 0..h {
                if i > 50 && j > 50 && i < 100 && j < 100 {
                    u_0[j*w + i] = 0x3fffffff
                }
            }
        }
            
       
        let v_0: Vec<i32> = vec![0; w*h]; 
        let force_0: Vec<i32> = vec![0; w*h];

        let image_0: Vec<u32> = u_0.iter().map(|&x| {toRGB(applyCap(x))}).collect(); 
        return Arena {
            height,
            width,
            image: image_0,
            u: u_0,
            v: v_0,
            force: force_0,
            status,
        };
    }

    #[no_mangle]
    pub fn step(&mut self, signalAmplitude: i32, dampingBitShift: u8) {
        // First loop: look for noise generator pixels and set their values in u:
        let w = self.width as usize;
        let h = self.height as usize;
        for i in (0..w * h) {
            if self.status[i] == PTRANSMITTER {
                self.u[i] = signalAmplitude;
                self.v[i] = 0;
                self.force[i] = 0;
            }
            if self.status[i] == NTRANSMITTER {
                self.u[i] = -signalAmplitude;
                self.v[i] = 0;
                self.force[i] = 0;
            }
        }
        // Second loop: apply wave equation at all pixels
        for i in (0..w * h) {
            if self.status[i] == DEFAULT {
                let uCen = self.u[i];
                let uNorth = self.u[i - w];
                let uSouth = self.u[i + w];
                let uEast = self.u[i + 1];
                let uWest = self.u[i - 1];
                let uxx = ((uWest + uEast) >> 1) - uCen;
                let uyy = ((uNorth + uSouth) >> 1) - uCen;
                let mut vel = self.v[i] + (uxx >> 1) + (uyy >> 1);
                if dampingBitShift > 0 {
                    vel -= (vel >> dampingBitShift);
                }
                self.v[i] = applyCap(vel);
            }
        }

        // Apply forces from the mouse:
        for i in 0..w * h {
            if self.status[i] == DEFAULT {
                let mut f = self.force[i];
                self.u[i] = applyCap(f + applyCap(self.u[i] + self.v[i]));
                f -= (f >> FORCE_DAMPING_BIT_SHIFT);
                self.force[i] = f;
            }
            if self.status[i] == WALL {
                self.image[i] = 0x00000000;
            } else {
                self.image[i] = toRGB(self.u[i]);
            }
        }
    }
    #[no_mangle]
    pub fn image(&self) -> *const u32 {
        self.image.as_ptr()
    }
    #[no_mangle]
    pub fn force(&self) -> *const i32 {
        self.force.as_ptr()
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
