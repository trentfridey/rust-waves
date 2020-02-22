//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate num;
use num::complex::Complex;

extern crate rust_waves;
use rust_waves::complex_to_rgba;
use rust_waves::Laboratory;

use std::f32;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_step() {
    let mut input_state = input_lab();
    let expected_state = expected_lab();
    input_state.step();
    for (input_psi, expected_psi) in input_state
        .get_psi()
        .into_iter()
        .zip(expected_state.get_psi().into_iter())
    {
        assert_eq!(input_psi, expected_psi)
    }
}

#[cfg(test)]
pub fn input_lab() -> Laboratory {
    // delta function spike at (x,y) = (50, 50)
    let mut lab = Laboratory::new();
    lab.delta_psi(&(50, 50));
    lab
}

#[cfg(test)]
pub fn expected_lab() -> Laboratory {
    /**
     * Stencil of evolution:
     *
     * Real
     *  0      -im      0
     *  -im   re+4im   -im
     *  0      -im      0
     *
     * Imaginary
     *  0       re      0
     *  re   -4re-im    re
     *  0       re      0
     */
    let mut lab = Laboratory::new();
    lab.set_psi(&[
        (50, 49, Complex { re: 0.0, im: 1.0 }),
        (50, 51, Complex { re: 0.0, im: 1.0 }),
        (49, 50, Complex { re: 0.0, im: 1.0 }),
        (51, 50, Complex { re: 0.0, im: 1.0 }),
        (50, 50, Complex { re: 1.0, im: -4.0 }),
    ]);
    lab
}

// #[wasm_bindgen_test]
// fn test_complex_to_rgba() {
//     /**
//      * Tests the conversion from complex to color hex code
//      * Target colors are
//      * Red - #ff0000ff
//      * Yellow - #ff00ffff
//      * Green - #ff00ff00
//      * Cyan - #ffffff00
//      * Blue - #ffff0000
//      * Magenta - #ffff00ff
//      *
//      * complex | color
//      *   1        Red
//      *   1+i      Yellow
//      *   -1+i     Green
//      *   -1       Cyan
//      *   -1-i     Blue
//      *   1-i      Magenta
//      *
//      */
//     let target_colors = vec![
//         0xff0000ff, 0xff00ffff, 0xff00ff00, 0xffffff00, 0xffff0000, 0xffff00ff,
//     ];
//     let input_complexes = vec![
//         Complex { re: 1f32, im: 0f32 },
//         Complex {
//             re: 0.5,
//             im: 3f32.sqrt() / 2.0,
//         },
//         Complex {
//             re: -0.5,
//             im: 3f32.sqrt() / 2.0,
//         },
//         Complex {
//             re: -1f32,
//             im: 0f32,
//         },
//         Complex {
//             re: -0.5,
//             im: -(3f32.sqrt()) / 2.0,
//         },
//         Complex {
//             re: 0.5,
//             im: -(3f32.sqrt()) / 2.0,
//         },
//     ];
//     for (complex, hex) in input_complexes.into_iter().zip(target_colors.into_iter()) {
//         assert_eq!(complex_to_rgba(complex), hex);
//     }
// }
