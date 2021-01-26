// Returns Vec<(dataPoint, frequency)>
pub fn get_frequencies(sorted_data: &[f32]) -> Vec<(f32, i32)> {
    if sorted_data.len() < 1 { return Vec::new() }
    let mut frequencies: Vec<(f32, i32)> = Vec::new();
    let mut previous: f32 = sorted_data[0];
    let mut count: i32 = 1;
    for i in 1..sorted_data.len() {
        if sorted_data[i] > previous {
            frequencies.push((previous, count));
            count = 1;
        }
        else {
            count += 1;
        }
        previous = sorted_data[i];
    }
    frequencies.push((previous, count));
    frequencies.sort_by_key(|k| k.1); // sort by freq
    frequencies
}

pub fn factorial(n: f64) -> f64 {
    let mut number: f64 = n;
    let mut result: f64 = 1.0;
    while number > 0.0 {
        result *= number;
        number = number - 1.0;
    }
    result
}