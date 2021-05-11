extern crate num;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod utils;
use utils::HexColor;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone)]
enum Status {
    Default,
    Wall,
}
const FORCE_DAMPING_BIT_SHIFT: u8 = 4;

#[wasm_bindgen]
pub struct Arena {
    width: u32,
    height: u32,
    u: Vec<i32>,
    v: Vec<i32>,
    force: Vec<i32>,
    image: Vec<u32>,
    status: Vec<Status>,
}

pub fn apply_cap(x: i32) -> i32 {
    if x < i32::MIN >> 1 {
        return i32::MIN >> 1;
    } else if x > i32::MAX >> 1 {
        return i32::MAX >> 1;
    } else {
        return x;
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
        let mut status = vec![Status::Default; w * h];
        let mut u_0: Vec<i32> = vec![0; w * h];
        // Draw walls along the outer boundary
        // left and right walls
        for i in 0..h {
            status[i * w] = Status::Wall;
            status[i * w + w - 1] = Status::Wall;
        }
        // top and bottom walls
        for j in 0..w {
            status[j] = Status::Wall;
            status[w * h - w + j] = Status::Wall;
        }

        // initial conditions
        for i in 0..w {
            for j in 0..h {
                if i > 50 && j > 50 && i < 100 && j < 100 {
                    u_0[j * w + i] = 0x3fffffff
                }
            }
        }

        let v_0: Vec<i32> = vec![0; w * h];
        let force_0: Vec<i32> = vec![0; w * h];

        let image_0: Vec<u32> = u_0.iter().map(|&x| apply_cap(x).to_rgba()).collect();
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
    pub fn step(&mut self,  damping_bit_shift: u8) {
        let w = self.width as usize;
        let h = self.height as usize;
        for i in 0..w * h {
            match self.status[i] {
                Status::Default => {
                    let u_cen = self.u[i];
                    let u_north = self.u[i - w];
                    let u_south = self.u[i + w];
                    let u_east = self.u[i + 1];
                    let u_west = self.u[i - 1];
                    let uxx = ((u_west + u_east) >> 1) - u_cen;
                    let uyy = ((u_north + u_south) >> 1) - u_cen;
                    let mut vel = self.v[i] + (uxx >> 1) + (uyy >> 1);
                    if damping_bit_shift > 0 {
                        vel -= vel >> damping_bit_shift;
                    }
                    self.v[i] = apply_cap(vel);
                },
                _ => {}
            }
        }
        // Apply forces from the mouse:
        for i in 0..w * h {
            match self.status[i] {
                Status::Default => {
                    let mut f = self.force[i];
                    self.u[i] = apply_cap(f + apply_cap(self.u[i] + self.v[i]));
                    f -= f >> FORCE_DAMPING_BIT_SHIFT;
                    self.force[i] = f;
                    self.image[i] = self.u[i].to_rgba();
                },
                Status::Wall => {
                    self.image[i] = 0x00000000;
                },
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
