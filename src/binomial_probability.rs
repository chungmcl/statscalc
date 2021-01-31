use super::general_funcs;
use super::arrangements;

pub fn binomial_probability(trials: i32, successes_in_trials: i32, probability_of_success: f32) -> f32 {
    let probability_of_failure: f32 = 1.0 - probability_of_success;
    let p_power: f32 = probability_of_success.powf(successes_in_trials as f32);
    let q_power: f32 = probability_of_failure.powf((trials - successes_in_trials) as f32);
    let ncx: f32 = arrangements::combinations(trials as f64, successes_in_trials as f64) as f32;
    ncx * p_power * q_power
}

pub fn binomial_probability_distribution(trials: i32, max_successes: i32, probability_of_success: f32) -> Vec<(i32, f32)> {
    let mut distribution: Vec<(i32, f32)> = Vec::new();
    for i in 0..(max_successes + 1) {
        distribution.push((i, binomial_probability(trials, i, probability_of_success)));
    }
    distribution
}

pub fn mean(trials: i32, probability_of_success: f32) -> f32 {
    trials as f32 * probability_of_success
}

pub fn variance(trials: i32, probability_of_success: f32) -> f32 {
    let probability_of_failure: f32 = 1.0 - probability_of_success;
    trials as f32 * probability_of_success * probability_of_failure
}

pub fn stdev(trials: i32, probability_of_success: f32) -> f32 {
    variance(trials, probability_of_success).sqrt()
}