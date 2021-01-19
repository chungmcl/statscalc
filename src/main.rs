#![allow(non_snake_case)] // hehe
#![allow(unused_parens)]
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
                let splitStringData: Vec<&str> = input.split(";").collect();
                for string in splitStringData {
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
    
        let centralTendencies: (f32, f32, f32) = centralTendency(&data);
        let measuresOfVariation: (f32, f32) = measuresOfVariation(&data);
        println!("Population StdDev: {}\nSample StdDev: {}", measuresOfVariation.0, measuresOfVariation.1);
        println!("Mean: {}\nMedian: {}\nMode: {}", centralTendencies.0, centralTendencies.1, centralTendencies.2);
        pause();
    }
}

// Returns (populationStandardDeviation, sampleStandardDeviation)
fn measuresOfVariation(data: &Vec<f32>) -> (f32, f32) {
    let centralTendency = centralTendency(&data);
    let mean: f32 = centralTendency.0;
    let mut sumOfSquares: f32 = 0.0;
    for i in data {
        sumOfSquares += ((i - mean) * (i - mean));
    }
    let populationStdDev: f32 = (sumOfSquares / data.len() as f32).sqrt();
    let sampleStdDev: f32 = (sumOfSquares / (data.len() - 1) as f32).sqrt();
    return (populationStdDev, sampleStdDev);
}

// Returns (mean, median, mode). If mode is uncalculatable, mode = NaNf32
fn centralTendency(data: &Vec<f32>) -> (f32, f32, f32) {
    let dataLen: usize = data.len();
    let mut sum: f32 = 0.0;
    for i in data {
        sum += i;
    }
    let mean: f32 = sum / dataLen as f32;

    let mut median: f32 = 0.0;
    if dataLen > 0 {
        if dataLen % 2 != 0 {
            median = data[(dataLen / 2)];
        }
        else {
            median = (data[(dataLen / 2)] + data[(dataLen / 2) - 1]) / 2.0;
        }
    }

    let frequencies: Vec<(f32, i32)> = getFrequencies(data);
    let lastFreq: (f32, i32) = frequencies[frequencies.len() - 1];
    let mode: f32;
    if lastFreq.1 > 1 { mode = lastFreq.0; }
    else { mode = (-1.0f32).sqrt(); } // NaN it to represent no mode

    return (mean, median, mode);
}

// Returns (dataPoint, frequency)
fn getFrequencies(data: &Vec<f32>) -> Vec<(f32, i32)> {
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
