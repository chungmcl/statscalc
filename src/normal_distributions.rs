
// From my informal testing, (+/-)5.0 * stdev seems to be the 
// best approximation for -inf and inf (in the case of a normal distribution)
const INFINITY_APPROXIMATION_COEFFICIENT: f32 = 5.0;

// From my informal testing, 1/10,000 for a coefficent of dx
// and also as n in the Riemann sum seems to give the
// closest approximation
const DX_APPROXIMATION_COEFFICIENT: f32 = 10000.0;

pub fn normal_distribution(x: f32, stdev: f32, mean: f32) -> f32 {
    std::f32::consts::E.powf((-1.0/2.0) * ((x - mean) / stdev).powf(2.0)) / (stdev * (2.0 * std::f32::consts::PI).sqrt())
}

// Approximate integral (Midpoint Riemann Sum) of a normal distribution
// Approximates -inf and inf to -5.0 * stdev & 5.0 * stdev respectively
// Midpoint Riemann Sum with n = 10,000.0
pub fn normal_distribution_integral(a: f32, b: f32, stdev: f32, mean: f32) -> f32 {
    let mut _a: f32 = a;
    let mut _b: f32 = b;
    
    
    if a == f32::NEG_INFINITY { _a = -1.0 * INFINITY_APPROXIMATION_COEFFICIENT * stdev };
    if b == f32::INFINITY { _b = INFINITY_APPROXIMATION_COEFFICIENT * stdev };

    let n: f32 = DX_APPROXIMATION_COEFFICIENT;
    let dx: f32 = (_b - _a) / n;

    let mut sum: f32 = 0.0;
    let mut i: f32 = 1.0;
    let mut xi: f32 = _a;
    while i <= n {
        sum += dx * normal_distribution(xi, stdev, mean);
        xi += dx;
        i += 1.0;
    }
    sum
}

pub fn normal_distribution_area_inverse(area: f32, stdev: f32, mean: f32) -> f32 {
    let dx: f32 = 1.0 / DX_APPROXIMATION_COEFFICIENT; // 1/10,000 seems to be the most accurate from my informal tests
    let mut sum: f32 = 0.0;
    let mut xi: f32 = -1.0 * INFINITY_APPROXIMATION_COEFFICIENT * stdev;
    while sum < area {
        sum += dx * normal_distribution(xi, stdev, mean);
        xi += dx;
    }
    xi
}