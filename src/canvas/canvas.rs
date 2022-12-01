///
/// Interface for interacting with a 2d grid of values `T`.
/// The library relies on this trait to render its output.
///
pub trait Canvas<T:Clone> {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_value(&self, x: usize, y: usize) -> T;
    fn set_value(&mut self, x: usize, y: usize, value: &T);
}
