use super::central_tendency;
use super::measures_of_variation;

// Returns (1st quartile, 2nd quartile, 3rd quartile, Interquartile range)
pub fn get_quartiles(sorted_data: &[f32]) -> (f32, f32, f32, f32) {
    let first_quartile: f32 = central_tendency::median(&sorted_data[0 .. (sorted_data.len() / 2)]);
    let second_quartile: f32 = central_tendency::median(sorted_data);

    let third_quartile_start_idx: usize = 
    if sorted_data.len() % 2 != 0 { (sorted_data.len() / 2) + 1 } 
    else { sorted_data.len() / 2 };

    let third_quartile: f32 = central_tendency::median(&sorted_data[third_quartile_start_idx .. sorted_data.len()]);
    (first_quartile, second_quartile, third_quartile, third_quartile - first_quartile)
}

pub fn standard_score(data: &[f32], value: f32, is_population: bool) -> f32 {
    let mean: f32 = central_tendency::mean(data);
    let stdev: f32 = measures_of_variation::stdev(data, is_population);
    (value - mean) / stdev
}