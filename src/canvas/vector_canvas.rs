use crate::canvas::canvas::Canvas;

///
/// Canvas backed by a vector.
///
pub struct VectorCanvas<T:Clone> {
    pub width: usize,
    pub height: usize,
    pub vector: Vec<T>,
}

impl<T:Clone> VectorCanvas<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> VectorCanvas<T> {
        let num_elements = width * height;
        let mut vector = Vec::with_capacity(num_elements);
        for _ in 0..num_elements {
            vector.push(default_value.clone());
        }
        VectorCanvas { width, height, vector }
    }

    #[inline(always)]
    pub fn get_flat_index(&self, x: usize, y: usize) -> usize {
        y * self.width +  x
    }
}

impl<T:Clone> Canvas<T> for VectorCanvas<T> {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_value(&self, x: usize, y: usize) -> T {
        let i = self.get_flat_index(x, y);
        self.vector[i].clone()
    }

    fn set_value(&mut self, x: usize, y: usize, value: &T) {
        let i = self.get_flat_index(x, y);
        self.vector[i] = value.clone();
    }

    // pub fn as_any(&self) -> &dyn Any {
    //     self
    // }

}


