use super::general_funcs;

pub fn get_all(sorted_data: &[f32]) -> (f32, f32, f32) {
    let mean: f32 = mean(&sorted_data);
    let median: f32 = median(&sorted_data);
    let mode: f32 = mode(&sorted_data);
    (mean, median, mode)
}

pub fn mean(data: &[f32]) -> f32 {
    let data_len: usize = data.len();
    let mut sum: f32 = 0.0;
    for i in data {
        sum += i;
    }
    sum / data_len as f32
}

pub fn median(sorted_data: &[f32]) -> f32 {
    let sorted_data_len: usize = sorted_data.len();
    let mut median: f32 = 0.0;
    if sorted_data_len > 0 {
        if sorted_data_len % 2 != 0 {
            median = sorted_data[(sorted_data_len / 2)]
        }
        else {
            median = (sorted_data[(sorted_data_len / 2)] + sorted_data[(sorted_data_len / 2) - 1]) / 2.0
        }
    }
    median
}

pub fn mode(sorted_data: &[f32]) -> f32 {
    let frequencies: Vec<(f32, i32)> = general_funcs::get_frequencies(&sorted_data);
    let last_freq: (f32, i32) = frequencies[frequencies.len() - 1];
    let mode: f32;
    if last_freq.1 > 1 { mode = last_freq.0; }
    else { mode = (-1.0f32).sqrt(); } // NaN it to represent no mode
    mode
}