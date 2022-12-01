pub fn map(value: f64, value_min: f64, value_max: f64, dest_min: f64, dest_max: f64) -> f64 {
    let ratio = (value - value_min) / (value_max - value_min);
    dest_min + (dest_max - dest_min) * ratio
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

// todo remove ugh
pub fn u8_add_clamped(a: u8, b: u8) -> u8 {
    let mut sum: u16 = a as u16 + b as u16;
    if sum > 255 {
        sum = 255
    }
    sum as u8
}


/// Is value within the given range (inclusive)
pub fn within(value: f64, min: f64, max: f64) -> bool {
    value >= min && value <= max
}