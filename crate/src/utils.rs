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

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const ALPHA: u32 = 0xFF_00_00_00; 
const FORCE_DAMPING_BIT_SHIFT: u8 = 4;
pub const I32_CAP_MIN: i32 = i32::MIN >> 1;
pub const I32_CAP_MAX: i32 = i32::MAX >> 1;

#[derive(Clone, Copy)]
pub enum Status {
    Default,
    Wall,
}
#[derive(Clone)]
pub struct Arena {
    pub width: u32,
    pub height: u32,
    pub status: Vec<Status>,
}

impl Arena {
    pub fn new(width: u32, height: u32) -> Arena {
        let w: usize = width as usize;
        let h: usize = height as usize;
        let status = (0..w * h).map(|x| {
            if x < w { return Status::Wall };                       // Top
            if (x + 1) % w == 0 { return Status::Wall };            // Right
            if h * (w - 1) < x && x < h * w { return Status::Wall };// Bottom
            if x % w == 0 { return Status::Wall };                  // Left
            return Status::Default;
        }).collect();

        return Arena {
            height,
            width,
            status,
        };
    }
    pub fn to_xy(&self, idx: usize) -> (i32, i32) {
        // returns coordinates with origin at center
        // up = +y, right = +x
        let w = self.width as usize;
        let h = self.height as usize;
        let (x0, y0) = (w / 2, h / 2);
        let y = y0 - idx / (self.width as usize);
        let x = (idx % (self.width as usize)) - x0;
        (x as i32, y as i32)
    }
    pub fn to_idx(&self, x: u32, y: u32) -> usize {
        (y * &self.width) as usize + x as usize
    }
}

pub struct CWave {
    u: Vec<i32>,
    v: Vec<i32>,
    force: Vec<i32>,
}
#[derive(Debug)]
pub struct QWave {
    psi: Vec<Complex<i32>>,
    pub norm: f32
}

pub trait Waveable {
    fn new(arena: &Arena) -> Self;
    fn step(&mut self, arena: &Arena, damping: u8);
    fn render(&self, norm_only: bool) -> Vec<u32>;
}

impl Waveable for CWave {
    fn new(arena: &Arena) -> CWave {
        let w: usize = arena.width as usize;
        let h: usize = arena.height as usize;
        let mut u_0: Vec<i32> = vec![0; w * h];
        for i in 0..w {
            for j in 0..h {
                if i > 50 && j > 50 && i < 100 && j < 100 {
                    u_0[j * w + i] = 0x3fffffff
                }
            }
        }
        let v_0: Vec<i32> = vec![0; w * h];
        let force_0: Vec<i32> = vec![0; w * h];
        return CWave {
            u: u_0,
            v: v_0,
            force: force_0,
        }
    }
    fn step(&mut self, arena: &Arena, damping: u8) {
        let w = arena.width as usize;
        let h = arena.height as usize;
        for i in 0..w * h {
            match arena.status[i] {
                Status::Default => {
                    let u_cen = self.u[i];
                    let u_north = self.u[i - w];
                    let u_south = self.u[i + w];
                    let u_east = self.u[i + 1];
                    let u_west = self.u[i - 1];
                    let uxx = ((u_west + u_east) >> 1) - u_cen;
                    let uyy = ((u_north + u_south) >> 1) - u_cen;
                    let mut vel = self.v[i] + (uxx >> 1) + (uyy >> 1);
                    if damping > 0 {
                        vel -= vel >> damping;
                    }
                    self.v[i] = vel.apply_cap();
                },
                _ => {}
            }
        }
        // Apply velocity and forces
        for i in 0..w * h {
            match arena.status[i] {
                Status::Default => {
                    let mut f = self.force[i];
                    self.u[i] = (f + (self.u[i] + self.v[i]).apply_cap()).apply_cap();
                    f -= f >> FORCE_DAMPING_BIT_SHIFT;
                    self.force[i] = f;
                },
                _ => {}
            }
        }
    } 
    fn render(&self, _norm_only: bool) -> Vec<u32> {
        return self.u.iter().map(|u: &i32| u.to_rgba(false)).collect();
    }
}

impl Waveable for QWave {
    fn new(arena: &Arena) -> QWave {
        let w: usize = arena.width as usize;
        let h: usize = arena.height as usize;

        let gauss = |x: i32, y: i32| -> f32 {
            let xf = x as f32 / arena.width as f32; 
            let yf = y as f32 / arena.height as f32;
            (-xf * xf - yf * yf).exp()
        };

        let sine = |x: i32, y: i32| -> f32 {
            let xf = x as f32 / arena.width as f32;
            let yf = y as f32 / arena.width as f32;
            xf.sin() * yf.sin()
        };

        let psi_0: Vec<Complex<i32>> = vec![Complex{re: 0, im: 0}; w * h]
            .into_iter()
            .enumerate()
            .map(|(i, pt)| {
                let (x, y) = arena.to_xy(i);
                let amp = gauss(20*x, 5*y)*10.0*sine(90, 90*y);
                let amp_i32 = (amp * (i32::MAX >> 4) as f32) as i32;
                return Complex{ re: amp_i32, im: 0 }
            }).collect();
        let norm_0 = psi_0.iter().fold(0.0, |acc, &psi| -> f32 {
            acc + Complex{ re: to_amp(psi.re), im: to_amp(psi.im) }.norm() / ((w * h) as f32)
        });
        return QWave {
            psi: psi_0,
            norm: norm_0
        }
    }
    fn step(&mut self, arena: &Arena, _damping: u8) {

        // finite difference algorithm

        let w = arena.width as usize;
        let h = arena.height as usize;

        let mut u_cen = self.psi.clone();
        let mut u_west: Complex<i32>;
        let mut u_east: Complex<i32>;
        let mut u_north: Complex<i32>;
        let mut u_south: Complex<i32>;
        let mut uxx = vec![Complex{re: 0, im: 0}; w * h];
        let mut uyy = vec![Complex{re: 0, im: 0}; w * h];

        for i in 0..w * h {
                match arena.status[i] {
                    Status::Default => {
                        u_cen[i] = self.psi[i];
                        u_west = self.psi[i - 1];
                        u_east = self.psi[i + 1];
                        u_north = self.psi[i - w];
                        u_south = self.psi[i + w];
                        uxx[i] = Complex{ re: ((u_west.re + u_east.re) >> 1 ), im: ((u_west.im + u_east.im) >> 1 )} - u_cen[i];
                        uyy[i] = Complex{ re: ((u_south.re + u_north.re) >> 1), im: ((u_south.im + u_north.im) >> 1)} - u_cen[i];
                    },
                    _ => {}
            }
        }
        for i in 0..w * h {
            match arena.status[i] {
                Status::Default => {
                    self.psi[i].re = u_cen[i].re - (uxx[i].im + uyy[i].im);
                    self.psi[i].im = u_cen[i].im - (uxx[i].re + uyy[i].re);
                    self.psi[i].apply_cap();
                },
                _ => {}
            }
        }
        self.norm = self.psi.iter().fold(0.0, |acc, &psi| -> f32 {
            acc + Complex{ re: to_amp(psi.re), im: to_amp(psi.im) }.norm() / ((w * h) as f32)
        });
        // let default_neighbors: Complex<i32> = left_neighbor + right_neighbor + top_neighbor + bottom_neighbor;
        // TODO: debug reflections
        // let psi_neighbors: Complex<i32> = match (row, col) {
        //     (0, 0) => default_neighbors +  (bottom_neighbor + right_neighbor) / 2,
        //     (0, col) if col == right => default_neighbors + (bottom_neighbor + left_neighbor) / 2,
        //     (0, _) => default_neighbors + bottom_neighbor,
        //     (row, 0) if row == bottom => default_neighbors +  (top_neighbor + right_neighbor) / 2,
        //     (row, col) if row == bottom && col == right => default_neighbors +  (top_neighbor + left_neighbor) / 2,
        //     (row, _) if row == bottom => default_neighbors + top_neighbor,
        //     (_, 0) => default_neighbors + right_neighbor,
        //     (_, col) if col == right => default_neighbors + left_neighbor,
        //     (_, _) => default_neighbors
        // };
    }
    fn render(&self, norm_only: bool) -> Vec<u32> {
        return self.psi.iter().map(|x| x.to_rgba(norm_only)).collect();
    }
}

pub trait Rectified {
    fn apply_cap(self) -> Self;
}

impl Rectified for i32 {
    fn apply_cap(self) -> i32 {
        if self < I32_CAP_MIN {
            return I32_CAP_MIN;
        } else if self > I32_CAP_MAX {
            return I32_CAP_MAX;
        } else {
            return self;
        }
    } 
}

impl Rectified for Complex<i32> {
    fn apply_cap(self) -> Complex<i32> {
        return Complex { re: self.re.apply_cap(), im: self.im.apply_cap() };
    }
}

// Convert i32 to float in [-1, 1]
pub fn to_amp (x: i32) -> f32 {
    return ((x as f32) + 0.5)/((I32_CAP_MAX) as f32 - 0.5)
}
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
    let (r,g,b) = ((255.0*r1) as u32 + m, (255.0*g1) as u32 + m, (255.0*b1) as u32 + m);
    return (r,g,b);
} 

pub fn gen_test_pattern(width: u32, height: u32) -> Vec<u32> {
    let (min, max) = (I32_CAP_MIN, I32_CAP_MAX);
    // map [0, width] -> [min, max]
    let sampling_scale = |x: u32| { 
        let s = (((max as f32 - min as f32)*(x as f32) / width as f32) as i32) + min;
        return s
    };     
    let mut test: Vec<u32> =  vec![0u32; (width * height) as usize];
    for x in 0..width {
        for y in 0..height {
            let index = (y*width + x) as usize;
            let complex_x = to_amp(sampling_scale(x));
            let arc: f32 = (1.0 - complex_x*complex_x).sqrt();
            let complex_y = -to_amp(sampling_scale(y));
            if -arc <= complex_y && complex_y < arc {
                test[index] = Complex { re: sampling_scale(x), im: sampling_scale(y) }.to_rgba(false);
            }
            else { 
                test[index] = 0xFF_000000;
            }
        }
    }
    return test
}

use std::f32::consts::PI; 

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

#[test]
pub fn test_delta_step () {
    let test_arena: Arena = Arena::new(5, 5);
    let mut test_wave: QWave = QWave::new(&test_arena);
    let mut result= vec![Complex{re: 0, im: 0}; 25];
    result[7].im =  -test_wave.psi[12].re >> 1;
    result[11].im = -test_wave.psi[12].re >> 1;
    result[13].im = -test_wave.psi[12].re >> 1;
    result[17].im = -test_wave.psi[12].re >> 1;
    result[12] = Complex{ re: test_wave.psi[12].re, im: 2*test_wave.psi[12].re};
    test_wave.step(&test_arena, 0);
    assert_eq!(result, test_wave.psi, "\n
    Actual:
    \n\t\t{}\n{} {} {} \n\t\t{}\n
    Expected:
    \n\t\t{}\n{} {} {} \n\t\t{}\n",
    result[7], 
    result[11], result[12], result[13], 
    result[17],
    test_wave.psi[7], 
    test_wave.psi[11], test_wave.psi[12], test_wave.psi[13], 
    test_wave.psi[17])
}

pub fn test_vline_step () {
    // TODO: implement
}

pub fn test_hline_step () {
    // TODO: implement
}

pub fn test_left_reflection () {
    // TODO: implement
}

pub fn test_right_reflection () {
    // TODO: implement
}