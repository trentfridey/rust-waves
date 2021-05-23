extern crate num;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


mod utils;
use utils::{Arena, QWave, CWave, Waveable, HexColor};
use num::Complex;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Canvas {
    arena: Arena,
    image: Vec<u32>,
    wave: QWave,
    test: Vec<u32>,
}


pub fn test_pattern() -> Vec<u32> {
    let (min, max) = (-0x40_000000 as i32, 0x3f_ffffff);
    let range = (min..max).len();
    let scale = |x: i32 | { (2.0*(x as f32) + 1.0) / (((1 << 31) as f32 - 1.0)) };


    let mut image: Vec<u32> =  vec![0u32; range];
    for (col, x) in (min..max).enumerate() {
        for (row, y) in (min..max).enumerate() {
            let index = row*range + col;
            let arc: f32 = (1.0 - scale(x)).sqrt();
            let yi = scale(y);
            if -arc <= yi && yi < arc {
                image[index] = Complex { re: x, im: y }.to_rgba();
            }
            else { 
                image[index] = 0xFF_000000;
            }
        }
    }
    return image;
}

#[wasm_bindgen]
impl Canvas { 
    #[no_mangle]
    pub fn new(width: u32, height: u32) -> Canvas {
        console_error_panic_hook::set_once();
        let arena = Arena::new(width, height);
        let image = vec![0xFF000000; (width*height) as usize];
        let wave: QWave = QWave::new(&arena);
        return Canvas {
            arena,
            image,
            wave,
            test: test_pattern()
        }
    }
    #[no_mangle]
    pub fn step(&mut self,  damping_bit_shift: u8) {
        self.wave.step(&self.arena, damping_bit_shift);
        self.image = self.wave.render(); 
    }
    #[no_mangle]
    pub fn image(&self) -> *const u32 {
        self.image.as_ptr()
    }
    #[no_mangle]
    pub fn test(&self) -> *const u32 {
        self.test.as_ptr()
    }
    #[no_mangle]
    pub fn width(&self) -> u32 {
        self.arena.width
    }
    #[no_mangle]
    pub fn height(&self) -> u32 {
        self.arena.height
    }
}
