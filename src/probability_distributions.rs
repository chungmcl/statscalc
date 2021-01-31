
pub fn mean(probability_distribution: &[(f32, f32)]) -> f32 {
    let mut mean: f32 = 0.0;
    for (x, p) in probability_distribution {
        mean += x * p;
    }
    mean
}

pub fn variance(probability_distribution: &[(f32, f32)]) -> f32 {
    let mean: f32 = mean(probability_distribution);
    let mut variance: f32 = 0.0;
    for (x, p) in probability_distribution {
        variance += (x - mean).powf(2.0) * p;
    }
    variance
}

pub fn stdev(probability_distribution: &[(f32, f32)]) -> f32 {
    variance(probability_distribution).sqrt()
}