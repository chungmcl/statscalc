
pub fn std_normal_distribution(x: f32, stdev: f32, mean: f32) -> f32 {
    (1.0 / (stdev * (2.0 * std::f32::consts::PI).sqrt())) * std::f32::consts::E.powf((-1.0/2.0) * ((x - mean) / stdev).powf(2.0))
}

pub fn std_normal_distribution_integral(stdev: f32, mean: f32, a: f32, b: f32) -> f32 {
    // If user passes in -infinity or +infinity I'll just approximate to -7 and 7
    let mut _a: f32 = a;
    let mut _b: f32 = b;
    if a == f32::NEG_INFINITY { _a = -10.0 };
    if b == f32::INFINITY { _b = 10.0 };

    let n: f32 = 1000000.0; // kind of arbritrary lol oh well
    let dx: f32 = (_b - _a) / n;

    let mut sum: f32 = 0.0;
    let mut i: f32 = 1.0;
    let mut xi: f32 = _a;
    while i <= n {
        sum += dx * std_normal_distribution(xi, stdev, mean);
        i += 1.0;
        xi += dx;
    }
    sum
}