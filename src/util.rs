pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

pub fn inverse_lerp(min: f64, max: f64, value: f64) -> f64 {
    (value - min) / (max - min)
}
