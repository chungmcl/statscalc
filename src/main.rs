use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut input: String = String::new();
        print!("Enter data delimited by semicolon: ");
        io::stdout().flush().unwrap();
    
        let mut data: Vec<f32> = Vec::new();
        // Switch between
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.pop(); // remove newline char
                let split_string_data: Vec<&str> = input.split(";").collect();
                for string in split_string_data {
                    data.push(string.parse::<f32>().unwrap());
                    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
                }
            },
            Err(e) => println!("Data entered is invalid: {}", e)
        }
        print!("Data entered in order: ");
        io::stdout().flush().unwrap();
        for i in &data {
            print!("{}, ", i);
            io::stdout().flush().unwrap();
        }
        println!();
    
        let central_tendencies: (f32, f32, f32) = central_tendency(&data);
        let measures_of_variation: (f32, f32) = measures_of_variation(&data);
        println!("Population StdDev: {}\nSample StdDev: {}", measures_of_variation.0, measures_of_variation.1);
        println!("Mean: {}\nMedian: {}\nMode: {}", central_tendencies.0, central_tendencies.1, central_tendencies.2);
        pause();
    }
}

fn measures_of_position(data: &Vec<f32>, fractiles: i32) {
    
}

// Returns (populationStandardDeviation, sampleStandardDeviation)
fn measures_of_variation(data: &Vec<f32>) -> (f32, f32) {
    let central_tendency = central_tendency(&data);
    let mean: f32 = central_tendency.0;
    let mut sum_of_squares: f32 = 0.0;
    for i in data {
        sum_of_squares += (i - mean) * (i - mean);
    }
    let population_stdev: f32 = (sum_of_squares / data.len() as f32).sqrt();
    let sample_stdev: f32 = (sum_of_squares / (data.len() - 1) as f32).sqrt();

    return (population_stdev, sample_stdev);
}

// Returns (mean, median, mode). If mode is uncalculatable, mode = NaNf32
fn central_tendency(data: &Vec<f32>) -> (f32, f32, f32) {
    let data_len: usize = data.len();
    let mut sum: f32 = 0.0;
    for i in data {
        sum += i;
    }
    let mean: f32 = sum / data_len as f32;

    let mut median: f32 = 0.0;
    if data_len > 0 {
        if data_len % 2 != 0 {
            median = data[(data_len / 2)];
        }
        else {
            median = (data[(data_len / 2)] + data[(data_len / 2) - 1]) / 2.0;
        }
    }

    let frequencies: Vec<(f32, i32)> = get_frequencies(data);
    let last_freq: (f32, i32) = frequencies[frequencies.len() - 1];
    let mode: f32;
    if last_freq.1 > 1 { mode = last_freq.0; }
    else { mode = (-1.0f32).sqrt(); } // NaN it to represent no mode

    return (mean, median, mode);
}

// Returns (dataPoint, frequency)
fn get_frequencies(data: &Vec<f32>) -> Vec<(f32, i32)> {
    if data.len() < 1 { return Vec::new() }

    let mut frequencies: Vec<(f32, i32)> = Vec::new();
    let mut previous: f32 = data[0];
    let mut count: i32 = 1;
    for i in 1..data.len() {
        if data[i] > previous {
            frequencies.push((previous, count));
            count = 1;
        }
        else {
            count += 1;
        }
        previous = data[i];
    }
    frequencies.push((previous, count));
    frequencies.sort_by_key(|k| k.1); // sort by freq

    return frequencies;
}

fn pause() {
    // pause
    io::stdin().read_line(&mut String::new()).unwrap();
    // clear console
    print!("\x1B[2J\x1B[1;1H");
}
