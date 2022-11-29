use crate::canvas::canvas::Canvas;
use crate::base::color::Color;

///
/// Color canvas backed by a u8 vector (rgba order)
///
pub struct U8Canvas {
    width: usize,
    height: usize,
    pub data: Vec<u8>
}

impl Canvas<Color> for U8Canvas {

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_value(&self, x: usize, y: usize) -> Color {
        let i = self.get_flat_index(x, y);
        Color::new(self.data[i], self.data[i + 1], self.data[i + 2])
    }

    fn set_value(&mut self, x: usize, y: usize, color: &Color) {
        let i = self.get_flat_index(x, y);
        self.data[i + 0] = color.r;
        self.data[i + 1] = color.g;
        self.data[i + 2] = color.b;
        self.data[i + 3] = 255;
    }

    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
}

impl U8Canvas {

    pub fn new(width: usize, height: usize) -> Self {
        let data:Vec<u8> = vec![0_u8; width * height * 4];
        U8Canvas { width, height, data }
    }

    #[inline(always)]
    fn get_flat_index(&self, x: usize, y: usize) -> usize {
        ((y * self.width) + x) * 4
    }
}
