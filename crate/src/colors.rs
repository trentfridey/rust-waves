extern crate num;
use num::complex::Complex;
use utils::{ALPHA, to_amp};
use std::f32::consts::PI; 


pub trait HexColor {
    fn to_rgba(self, norm_only: bool) -> u32;
}

impl HexColor for i32 {
    fn to_rgba(self, norm_only: bool) -> u32 {
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
    fn to_rgba(self, norm_only: bool) -> u32 {
        let f: Complex<f32> = Complex{ re: to_amp(self.re), im: to_amp(self.im) };
        let value = f.norm(); 
        if norm_only {
            let (v, _, __) = hsv_to_rgb(0.0, 1.0, value);
            return ALPHA | v << 16 | v << 8 | v
        }
        let hue_in_rads = f.arg();
        let hue_in_degs = hue_in_rads * 180.0 / PI; 
        let (r,g,b) = hsv_to_rgb(hue_in_degs, 1.0, value);
        return ALPHA | b << 16 | g << 8 | r
    } 
}


pub fn hsv_to_rgb(hue: f32, sat: f32, val: f32) -> (u32, u32, u32) {
    // based on algorithm from https://en.wikipedia.org/wiki/HSL_and_HSV
    // assume H in [0,360], S in [0,1], V in [0,1]
    // we handle H < 0 because it often is from atan2, which is in [-pi, pi) 
    let hue_rect = if hue < 0.0 { hue + 360.0 } else { hue };
    let chroma = sat * val.sqrt();
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
    let (r,g,b) = ((255.0*r1) as u32 + m, (255.0*g1) as u32 + m, (255.0*b1) as u32 + m);
    return (r,g,b);
} 

#[test]
pub fn test_to_rgba_red () {
    const RED: u32 = 0xFF_00_00_FF;
    const UNITY: Complex<i32> = Complex {re: 1 << 30, im: 0};
    let result = UNITY.to_rgba(false);
    assert_eq!(result, RED, "\nExpected: 0x{:X}\nActual:   0x{:X}", RED, result);
}

#[test]
pub fn test_to_rgba_green () {
    const GREEN: u32 = 0xFF_00_FF_00;
    let one_third_turn: Complex<i32> = Complex {re: -(1 << 29), im: (((1 << 29) as f32)*((3.0_f32.sqrt()))) as i32};
    let result = one_third_turn.to_rgba(false);
    assert_eq!(result, GREEN, "\nExpected: 0x{:X}\nActual:   0x{:X}", GREEN, result);
}

#[test]
pub fn test_to_rgba_blue () {
    const BLUE: u32 = 0xFF_FF_00_00;
    let two_third_turn: Complex<i32> = Complex {re: -(1 << 29), im: -(((1 << 29) as f32)*((3.0_f32.sqrt()))) as i32};
    let result = two_third_turn.to_rgba(false);
    assert_eq!(result, BLUE, "\nExpected: 0x{:X}\nActual:   0x{:X}", BLUE, result);
}

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
    let one_third_turn: Complex<f32> = Complex { re: -0.5, im: 0.5*(3.0_f32.sqrt()) };
    let result = hsv_to_rgb(one_third_turn.arg() * 180.0/ PI, 1.0, one_third_turn.norm());
    assert_eq!(ALPHA | result.0 << 16 | result.1 << 8 | result.2, GREEN);
}

#[test]
pub fn test_blue () {
    const BLUE: u32  = 0xFF_00_00_FF;
    let two_third_turn: Complex<f32> = Complex { re: -0.5, im: -0.5*(3.0_f32.sqrt()) };
    let result = hsv_to_rgb(two_third_turn.arg() * 180.0 / PI, 1.0, two_third_turn.norm());
    assert_eq!(ALPHA | result.0 << 16 | result.1 << 8 | result.2, BLUE);
}

#[test]
pub fn test_black () {
    const BLACK: u32 = 0xFF_00_00_00;
    const ORIGIN: Complex<f32> = Complex { re: 0.0, im: 0.0 };
    let result = hsv_to_rgb(ORIGIN.arg(), 1.0, ORIGIN.norm());
    assert_eq!(ALPHA | result.0 << 16 | result.1 << 8 | result.2, BLACK);
}