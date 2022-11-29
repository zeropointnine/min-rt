use std::ops;
use crate::util::maths;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }

    pub fn set_using_color(&mut self, color: &Color) {
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
    }
}

// operator overloads

// Color Color
impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        let r: u8 = maths::u8_add_clamped(self.r, rhs.r);
        let g: u8 = maths::u8_add_clamped(self.g, rhs.g);
        let b: u8 = maths::u8_add_clamped(self.b, rhs.b);
        Color::new(r, g, b)
    }
}

// Color f32
impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        let r: f32 = maths::clamp(self.r as f32 * rhs, 0.0, 255.0);
        let g: f32 = maths::clamp(self.g as f32 * rhs, 0.0, 255.0);
        let b: f32 = maths::clamp(self.b as f32 * rhs, 0.0, 255.0);
        Color::new(r as u8, g as u8, b as u8)
    }
}

// &Color f32
impl ops::Mul<f32> for &Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        let r: f32 = maths::clamp(self.r as f32 * rhs, 0.0, 255.0);
        let g: f32 = maths::clamp(self.g as f32 * rhs, 0.0, 255.0);
        let b: f32 = maths::clamp(self.b as f32 * rhs, 0.0, 255.0);
        Color::new(r as u8, g as u8, b as u8)
    }
}