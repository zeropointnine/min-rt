use std::ops;
use crate::util::maths;

/// Library's color class
/// Values are expected to be between 0.0 and 1.0
/// Any math operations get clamped to that range. // todo revisit; need guards on setters etc?
///
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn new_black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn from_u8(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: (r as f64) / 255.0,
            g: (g as f64) / 255.0,
            b: (b as f64) / 255.0
        }
    }

    pub fn set(&mut self, r: f64, g: f64, b: f64) { // todo clamp to legal range?
        self.r = r;
        self.g = g;
        self.b = b;
    }

    pub fn set_using_color(&mut self, color: &Color) {
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        let r = (self.r * 255.0) as u8;  // todo verify off-by-1 related
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;
        (r, g, b)
    }
}

// operator overloads

// Color Color
impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        let mut r = self.r + rhs.r;
        if r > 1.0 {
            r = 1.0;
        }
        let mut g = self.g + rhs.g;
        if g > 1.0 {
            g = 1.0;
        }
        let mut b = self.b + rhs.b;
        if b > 1.0 {
            b = 1.0;
        }
        Color::new(r, g, b)
    }
}

// Color f64
impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        let mut r = self.r * rhs;
        if r > 1.0 {
            r = 1.0;
        }
        let mut g = self.g * rhs;
        if g > 1.0 {
            g = 1.0;
        }
        let mut b = self.b * rhs;
        if b > 1.0 {
            b = 1.0;
        }
        Color::new(r, g, b)
    }
}

// &Color f64
impl ops::Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        let r = self.r * rhs;
        let mut r = maths::clamp(r, 0.0, 1.0);
        if r > 1.0 {
            r = 1.0;
        }
        let mut g = self.g * rhs;
        if g > 1.0 {
            g = 1.0;
        }
        let mut b = self.b * rhs;
        if b > 1.0 {
            b = 1.0;
        }
        Color::new(r, g, b)
    }
}