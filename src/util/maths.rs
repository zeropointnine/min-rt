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

// todo test if this is even necessary
pub fn u8_add_clamped(a: u8, b: u8) -> u8 {
    let mut sum: u16 = a as u16 + b as u16;
    if sum > 255 {
        sum = 255
    }
    sum as u8
}


/// Is value within the given range (inclusive)
pub fn within(value: f32, min: f32, max: f32) -> bool {
    value >= min && value <= max
}