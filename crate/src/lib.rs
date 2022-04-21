extern crate num;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


mod utils;
use utils::{Arena, gen_test_pattern};

mod colors;
mod waves;
use waves::{QWave, CWave, Waveable};
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

#[wasm_bindgen]
pub struct Canvas {
    arena: Arena,
    image: Vec<u32>,
    wave: CWave,
    test: Vec<u32>,
}

#[wasm_bindgen]
impl Canvas { 
    #[no_mangle]
    pub fn new(width: u32, height: u32) -> Canvas {
        console_error_panic_hook::set_once();
        let arena = Arena::new(width, height);
        let image = vec![0xFF000000; (width*height) as usize];
        let wave: CWave = CWave::new(&arena);
        let test: Vec<u32> = gen_test_pattern(arena.width, arena.height);
        
        return Canvas {
            arena,
            image,
            wave,
            test,
        }
    }
    #[no_mangle]
    pub fn step(&mut self,  damping_bit_shift: u8, norm_only: bool) {
        self.wave.step(&self.arena, damping_bit_shift);
        self.image = self.wave.render(norm_only); 
    }
    #[no_mangle]
    pub fn image(&self) -> *const u32 {
        self.image.as_ptr()
    }
    #[no_mangle]
    pub fn test(&self) -> *const u32 {
        self.test.as_ptr()
    }
    // #[no_mangle]
    // pub fn norm(&self) -> f32 {
    //     self.wave.norm
    // }
    #[no_mangle]
    pub fn width(&self) -> u32 {
        self.arena.width
    }
    #[no_mangle]
    pub fn height(&self) -> u32 {
        self.arena.height
    }
}
