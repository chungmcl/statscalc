use super::central_tendency;

pub fn get_all(data: &[f32]) -> (f32, f32) {
    let population_stdev = stdev(&data, true);
    let sample_stdev = stdev(&data, false);
    (population_stdev, sample_stdev)
}

pub fn stdev(data: &[f32], is_population: bool) -> f32 {
    let set_count: f32 = data.len() as f32 + if is_population { 0.0 } else { -1.0 };
    (sum_of_squares(&data) / set_count).sqrt()
}

pub fn deviation(data: &[f32], x: f32) -> f32 {
    x - central_tendency::mean(&data)
}

fn sum_of_squares(data: &[f32]) -> f32 {
    let mean: f32 = central_tendency::mean(&data);
    let mut sum_of_squares: f32 = 0.0;
    for i in data {
        sum_of_squares += (i - mean).powf(2.0);
    }
    sum_of_squares
}

