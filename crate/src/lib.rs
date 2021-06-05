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




#[wasm_bindgen]
impl Canvas { 
    #[no_mangle]
    pub fn new(width: u32, height: u32) -> Canvas {
        console_error_panic_hook::set_once();
        let arena = Arena::new(width, height);
        let image = vec![0xFF000000; (width*height) as usize];
        let wave: QWave = QWave::new(&arena);
        
        let (min, max) = (i32::MIN >> 1, i32::MAX >> 1); // full range of possible colors
        // map [0, width] -> [min, max]
        let sampling_scale = |x: u32| { 
            let s = (((max as f32 - min as f32)*(x as f32) / width as f32) as i32) + min;
            return s
        }; 
        // map [min, max] -> [-1,1]
        let scale = |x: i32| { 
            return ((x as f32) + 0.5)/((i32::MAX >> 1) as f32 - 0.5) 
        };
    
        let mut test: Vec<u32> =  vec![0u32; (width * height) as usize];
        for x in 0..width {
            for y in 0..height {
                let index = (y*width + x) as usize;
                let complex_x = scale(sampling_scale(x));
                let arc: f32 = (1.0 - complex_x*complex_x).sqrt();
                let complex_y = -scale(sampling_scale(y));
                if -arc <= complex_y && complex_y < arc {
                    test[index] = Complex { re: sampling_scale(x), im: sampling_scale(y) }.to_rgba();
                }
                else { 
                    test[index] = 0xFF_000000;
                }
            }
        }
        return Canvas {
            arena,
            image,
            wave,
            test,
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
