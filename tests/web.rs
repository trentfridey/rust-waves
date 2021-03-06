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
fn test_step_stability() {
    let mut numerical = Laboratory::new();
    let mut analytical = Laboratory::new();
    for (numerical_psi, analytical_psi) in numerical
        .get_psi()
        .into_iter()
        .zip(analytical.get_psi().into_iter()){
            assert_eq!(numerical_psi, analytical_psi)
        }
    numerical.step();
    analytical.exact_step();
    const EPSILON: f32 = 1e-6;
    for (numerical_psi, analytical_psi) in numerical
        .get_psi()
        .into_iter()
        .zip(analytical.get_psi().into_iter())
    {
        assert!((numerical_psi.re - analytical_psi.re) < EPSILON, "{} - {}", numerical_psi.re, analytical_psi.re);
        assert!((numerical_psi.im - analytical_psi.im) < EPSILON, "{}i - {}i", numerical_psi.im, analytical_psi.im);
    }
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
