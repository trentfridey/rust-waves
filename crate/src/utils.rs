extern crate num;
use num::complex::Complex;
//     // When the `console_error_panic_hook` feature is enabled, we can call the
//     // `set_panic_hook` function at least once during initialization, and then
//     // we will get better error messages if our code ever panics.
//     //
//     // For more details see
//     // https://github.com/rustwasm/console_error_panic_hook#readme
//     #[cfg(feature = "console_error_panic_hook")]
//     console_error_panic_hook::set_once();
// }

const ALPHA: u32 = 0xFF_00_00_00; 

pub trait HexColor {
    fn to_rgba(self) -> u32;
}

impl HexColor for i32 {
    fn to_rgba(self) -> u32 {
        let mut val: i32 = self >> 22;
        if val > 0 {
            let res = val as u32;
            return (res << 8) | (res << 16) | ALPHA;
        } else {
            val = std::cmp::max(val, -255_i32);
            return (-1_i32 * val) as u32 | ALPHA;
        }
    }
}

impl HexColor for Complex<i32> {
    fn to_rgba(self) -> u32 {
        let f: Complex<f32> = Complex{ re: self.re as f32, im: self.im as f32};
        let hue = f.arg(); // should this cast an i32 to a float?
        let value = f.norm(); // should this also cast to a float?
        let (r,g,b) = hsv_to_rgb(hue, 1.0, value);
        return ALPHA | r << 16 | g << 8 | b
    } 
}

pub fn hsv_to_rgb(hue: f32, sat: f32, val: f32) -> (u32, u32, u32) {
    // based on algorithm from https://en.wikipedia.org/wiki/HSL_and_HSV
    // assume H in [0,360], S in [0,1], V in [0,1]
    // we handle H < 0 because it often is from atan2, which is in [-pi, pi) 
    let hue_rect = if hue < 0.0 { hue + 360.0 } else { hue };
    let chroma = sat * val;
    let hue_div = hue_rect / 60.0;

    let x = chroma * (1.0-((hue_div % 2.0) - 1.0).abs()); 
    let (r1, g1, b1) = match hue_div {
        h if h < 1.0 => (chroma, x, 0.0),
        h if h < 2.0 => (x, chroma, 0.0),
        h if h < 3.0 => (0.0, chroma, x),
        h if h < 4.0 => (0.0, x, chroma),
        h if h < 5.0 => (x, 0.0, chroma),
        h if h < 6.0 => (chroma, 0.0, x),
        _ => (0.0, 0.0, 0.0)
    };
    let m: u32 = (val - chroma) as u32;
    let (r,g,b) = (255*(r1 as u32 + m), 255*(g1 as u32 + m), 255*(b1 as u32 + m));
    return (r,g,b);
}

use std::f32::consts::{FRAC_1_SQRT_2, PI}; 

#[test]
pub fn test_red () {
    const RED: u32   = 0xFF_FF_00_00;
    const UNITY: Complex<f32> = Complex { re: 1.0, im: 0.0 };
    let result = hsv_to_rgb(UNITY.arg(), 1.0, UNITY.norm());
    assert_eq!(ALPHA | result.0 << 16 | result.1 << 8 | result.2, RED);
}

#[test]
pub fn test_green () {
    const GREEN: u32 = 0xFF_00_FF_00;
    let ONE_THIRD_TURN: Complex<f32> = Complex { re: -0.5, im: 0.5*(3.0_f32.sqrt()) };
    let result = hsv_to_rgb(ONE_THIRD_TURN.arg() * 180.0/ PI, 1.0, ONE_THIRD_TURN.norm());
    assert_eq!(ALPHA | result.0 << 16 | result.1 << 8 | result.2, GREEN);
}

#[test]
pub fn test_blue () {
    const BLUE: u32  = 0xFF_00_00_FF;
    let TWO_THIRD_TURN: Complex<f32> = Complex { re: -0.5, im: -0.5*(3.0_f32.sqrt()) };
    let result = hsv_to_rgb(TWO_THIRD_TURN.arg() * 180.0 / PI, 1.0, TWO_THIRD_TURN.norm());
    assert_eq!(ALPHA | result.0 << 16 | result.1 << 8 | result.2, BLUE);
}

#[test]
pub fn test_black () {
    const BLACK: u32 = 0xFF_00_00_00;
    const ORIGIN: Complex<f32> = Complex { re: 0.0, im: 0.0 };
    let result = hsv_to_rgb(ORIGIN.arg(), 1.0, ORIGIN.norm());
    assert_eq!(ALPHA | result.0 << 16 | result.1 << 8 | result.2, BLACK);
}
