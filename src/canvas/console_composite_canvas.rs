use std::cmp;
use crate::canvas::canvas::Canvas;
use crate::canvas::vector_canvas::VectorCanvas;
use crate::util::ansi;
use crate::base::color::Color;

///
/// Designed for rendering console output, where each 'character cell' has a bg color and a char.
/// Uses two `Canvas`'es for this purpose (does not implement `Canvas` directly).
///
pub struct ConsoleCompositeCanvas {
    pub width: usize,
    pub height: usize,
    pub colors: VectorCanvas<Color>,
    pub chars: VectorCanvas<char>,
    pub text_color: Color
}

impl ConsoleCompositeCanvas {

    pub fn new(width: usize, height: usize, text_color: Color) -> ConsoleCompositeCanvas {
        let colors = VectorCanvas::<Color>::new(width, height, Color::new_black());
        let chars = VectorCanvas::<char>::new(width, height, ' ');
        ConsoleCompositeCanvas { width, height, colors, chars, text_color }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn clear_chars(&mut self, char: char) {
        for item in &mut self.chars.vector {
            *item = char;
        }
    }

    /// 'Fills' a row in the char canvas with chars from a string at the given position
    pub fn set_text(&mut self, x: usize, y: usize, string: &str) {

        let end_x =  cmp::min(x + string.len(), self.width);
        let num_chars = end_x - x;

        let vector_index = self.chars.get_flat_index(x, y);

        let mut char_iter = string.chars();

        for i in 0..num_chars {
            let char = char_iter.next().unwrap();
            self.chars.vector[vector_index + i] = char;
        }
    }

    // Outputs the colors + chars to the console using truecolor ANSI codes.
    pub fn print_to_console(&self) {

        print!("{}", ansi::CODE_HIDE_CURSOR);

        // Note, clearing the screen beforehand is unnecessary and introduces flicker.
        // Instead, we'll just be overwriting everything that may already be on screen.
        print!("{}", ansi::move_cursor(0, 0));

        let (r, g, b) = self.text_color.to_u8();
        print!("{}", ansi::foreground_color(r, g, b));

        for iy in 0..self.height {

            let mut row_text: String = String::new();

            for ix in 0..self.width {
                // Add ANSI background color command
                let color = self.colors.get_value(ix, iy);
                let (r, g, b) = color.to_u8();
                let code = ansi::background_color(r, g, b);
                row_text += &code;
                // Add the literal character
                let char = self.chars.get_value(ix, iy);
                row_text.push(char.clone());
            }

            // print the row
            let is_last_row = iy == self.height - 1;
            if !is_last_row {
                println!("{}", row_text);
            } else {
                print!("{}", row_text);
                print!("{}", ansi::move_cursor(0, 0));
            }

            // todo performance
            // stdout::write each row instead of using macro?
            // consider printing per-character rather than per-row, skipping String concatenation
            // do benchmarks
        }

        // todo 'grayscale' ascii style?
        // todo 'quadrant' characters for 2x rez heh
    }
}
