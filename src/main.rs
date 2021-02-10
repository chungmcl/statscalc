use std::io;
use std::io::Write;

mod general_funcs;
mod central_tendency;
mod measures_of_variation;
mod measures_of_position;
mod arrangements;
mod probability_distributions;
mod binomial_probability;
mod normal_distributions;

fn main() {
    loop {
        let mut continue_execution: bool = true;
	    println!("statscalc");
        println!("0. Exit");
        println!("1. Data Set Info");
        println!("2. Arrangments");
        println!("3. Probability Distribution Info");
        println!("4. Binomial Probability Info");
        println!("5. Normal Distribution Info");

        let input: &str = &get_input("Enter Selection")[..];
        clear();
        match input {
            "0" => { continue_execution = false },
            "1" => data_set_info(),
            "2" => arrangements_info(),
            "3" => probability_distribution_info(),
            "4" => binomial_probability_info(),
            "5" => normal_distribution_info(),
            _ => { println!("Invalid selection. Please try again."); }
        }
        if continue_execution { pause(); }
        else { break; }
    }
}

// TO DO: Implement way for user to enter an X and find its deviation
fn data_set_info() {
    println!("DATA SET INFO");
    println!("Calculates central tendencies, measures of position, and measures of variation");
    print!("Enter data delimited by ; or , : ");
    io::stdout().flush().unwrap(); // print requires you to flush the buffer yourself, but not println

    let mut data: Vec<f32> = Vec::new();

    let input: Vec<String> = get_split_input("Input");
    for string in input {
        data.push(string.parse::<f32>().unwrap());
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    print!("Data entered in order: ");
    io::stdout().flush().unwrap();
    for i in &data {
        print!("{}, ", i);
        io::stdout().flush().unwrap();
    }
    println!();

    let central_tendencies: (f32, f32, f32) = central_tendency::get_all(&data);
    let measures_of_variation: (f32, f32) = measures_of_variation::get_all(&data);
    let quartiles: (f32, f32, f32, f32) = measures_of_position::get_quartiles(&data);

    println!("Population StdDev: {}\nSample StdDev: {}", measures_of_variation.0, measures_of_variation.1);
    println!("Mean: {}\nMedian: {}\nMode: {}", central_tendencies.0, central_tendencies.1, central_tendencies.2);
    println!("Quartiles: 1st = {}, 2nd = {}, 3rd = {}, IQR = {}", quartiles.0, quartiles.1, quartiles.2, quartiles.3);
}

fn arrangements_info() {
    println!("ARRANGEMENTS");
    println!("Calculates permutations and combinations for a given n and r");
    println!("(Enter only n for # of different permutations of n distinct objects)");
    print!("Enter n and r as \"n;r\" OR \"n,r\"");

    let input: Vec<String> = get_split_input("Input: ");
    if input.len() == 2 {
        let n: f64 = input[0].parse::<f64>().unwrap();
        let r: f64 = input[1].parse::<f64>().unwrap();
        println!("Permutations: {}", arrangements::permutations_nr(n, r));
        println!("Combinations: {}", arrangements::combinations(n, r));
    }
    else if input.len() == 1 {
        let n: f64 = input[0].parse::<f64>().unwrap();
        println!("Permutations: {}", arrangements::permutations_n(n));
    }
    else {
        println!("Invalid input");
    }
}

fn probability_distribution_info() {
    println!("PROBABILITY DISTRIBUTION INFO");
    println!("Gives info on mean, variance, and standard deviation of a probability distribution");
    print!("Enter x and its respective p as x;p and separate each pair with ,");
    let input: String = get_input("Input");

    let pairs: Vec<&str> = input.split(';').collect();
    let mut data: Vec<(f32, f32)> = Vec::new();
    for pair in pairs {
        let pair_split: Vec<&str> = pair.split(',').collect();
        let x: f32 = pair_split[0].parse::<f32>().unwrap();
        let y: f32 = pair_split[1].parse::<f32>().unwrap();
        data.push((x, y));
    }

    let mean: f32 = probability_distributions::mean(&data);
    let stdev: f32 = probability_distributions::stdev(&data);
    println!("Mean: {}", mean);
    println!("Stdev: {}", stdev);
}

// Super buggy rn
fn binomial_probability_info() {
    loop {
        let mut continue_execution: bool = true;
	    println!("BINOMIAL PROBABILITY");
        println!("0. Exit");
        println!("1. Binomial Probability Formula");
        println!("2. Binomial Probability Distribution Generator");
        let selection: &str = &get_input("Enter selection")[..];

        clear();
        match selection {
            "0" => { continue_execution = false },
            "1" => { // Binomial Probability Formula
                println!("Enter # of trials, # of successes, and probability of success\nin this order separated by , or ;");
                let input: Vec<String> = get_split_input("Input: ");
                let mut bpf_data: Vec<f32> = Vec::new();
                for string in input {
                    bpf_data.push(string.parse::<f32>().unwrap());
                    bpf_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
                }
                let result: f32 = binomial_probability::binomial_probability(bpf_data[0] as i32, bpf_data[1] as i32, bpf_data[2]);
                println!("Result: {}", result);
            },
            "2" => { // Binomial Probability Distribution Generator
                let trials: i32 = get_input("Enter # of trials").parse::<i32>().unwrap();
                let max_successes: i32 = get_input("Enter max # of successes").parse::<i32>().unwrap();
                let probability_of_success: f32 = get_input("Enter probability of success").parse::<f32>().unwrap();

                let result: Vec<(i32, f32)> = binomial_probability::binomial_probability_distribution(trials, max_successes, probability_of_success);
                let mut result_string: String = String::from("x\tp\n");
                for i in result {
                    result_string.push_str(&(i.0).to_string()); 
                    result_string.push_str("\t");
                    result_string.push_str(&(i.1).to_string());
                    result_string.push_str("\n");
                }
                println!("{}", result_string);
            },
            _ => { println!("Invalid selection. Please try again."); }
        }
        if continue_execution { pause(); }
        else { break; }
    }
}

fn normal_distribution_info() {
    println!("NORMAL DISTRIBUTION INFO");
    println!("0. Exit");
    println!("1. Find Y of Z value");
    println!("2. Find area under curve");
    println!("3. Find the Z of an area");
    let selection: &str = &get_input("Enter Selection")[..];

    clear();
    match selection {
        "0" =>  { /* do nothing and exit */ },
        "1" => { // Find Y of Z value

        },
        "2" => {
            println!("Approximate integral of a normal distribution");
            println!("Enter -- a, b, stdev, mean -- in this order separated by , or ;");
            println!("For a standard normal distribution, enter a and b only");
            println!("Infinity can be represented by -inf and inf");
            let input: Vec<f32> = get_split_f32_input("Input");
            let area: f32;
            if input.len() > 2 {
                area = normal_distributions::normal_distribution_integral(input[0], input[1], input[2], input[3]);
            }
            else {
                area = normal_distributions::normal_distribution_integral(input[0], input[1], 1.0, 0.0);
            }
            let p: f32 = area * 100.0;
            // :04 -> total of four digits -- .2 -> two decimal places
            println!("The area is: {} ≈ {:04.2}%", area, p);
        }
        "3" => {
            println!("Approximate Z value of a given area");
            println!("Enter -- area, stdev, mean -- in this order separated by , or ;");
            println!("For a standard normal distribution, enter area only");
            let input: Vec<f32> = get_split_f32_input("Input");
            let z: f32;
            if input.len() > 1 {
                z = normal_distributions::normal_distribution_area_inverse(input[0], input[1], input[2]);
            }
            else {
                z = normal_distributions::normal_distribution_area_inverse(input[0], 1.0, 0.0);
            }
            println!("Closest approximation of Z ≈ {} ≈ {:04.4}", z, z);
        }
        _ => { println!("Invalid selection. Please try again."); }
    }
}

fn get_split_f32_input(message: &str) -> Vec<f32> {
    let mut result: Vec<f32> = Vec::new();
    let strings: Vec<String> = get_split_input(message);
    for i in strings {
        result.push(i.parse::<f32>().unwrap());
    }
    result
}

fn get_split_input(message: &str) -> Vec<String> {
    let mut result: Vec<String> = get_input(message).split(|c| c == ',' || c == ';').map(String::from).collect();
    for i in 0..result.len() {
        result[i] = String::from(result[i].trim());
    }
    result
}

fn get_input(message: &str) -> String {
    // print requires explicit clearing of buffer
    print!("{}: ", message); io::stdout().flush().unwrap(); 

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Input Failed!"); input.pop();
    String::from(input.trim())
}

fn pause() {
    io::stdin().read_line(&mut String::new()).unwrap();
    clear();
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
