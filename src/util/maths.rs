pub fn map(value: f32, value_min: f32, value_max: f32, new_min: f32, new_max: f32) -> f32 {
    let ratio = (value - value_min) / (value_max - value_min);
    new_min + (new_max - new_min) * ratio
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        return min;
    } else if value > max {
        return max;
    } else {
        value
    }
}