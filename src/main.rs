use std::io;
use std::io::Write;

mod general_funcs;
mod central_tendency;
mod measures_of_variation;
mod measures_of_position;
mod arrangements;
mod probability_distributions;
mod binomial_probability;

fn main() {
    loop {
        let mut continue_execution: bool = true;
	    println!("statscalc");
        let mut input: String = String::new();
        println!("0. Exit");
        println!("1. Data Set Info");
        println!("2. Arrangments");
        println!("3. Probability Distribution Info");
        println!("4. Binomial Probability Info");
        
        print!("Enter Selection: ");
        io::stdout().flush().unwrap(); // print requires you to flush the buffer yourself, but not println

        io::stdin().read_line(&mut input).expect("Input Failed!");
        input.pop(); // Remove newline from stdin.read_line()
        let trimmed_input: &str = input.trim();
        clear();
        match trimmed_input {
            "0" => { continue_execution = false },
            "1" => data_set_info(),
            "2" => arrangements_info(),
            "3" => probability_distribution_info(),
            "4" => binomial_probability_info(),
            _ => { println!("Invalid selection. Please try again."); }
        }
        if continue_execution { pause(); }
        else { break; }
    }
}

// TO DO: Implement way for user to enter an X and find its deviation
fn data_set_info() {
    let mut input: String = String::new();
    println!("DATA SET INFO");
    println!("Calculates central tendencies, measures of position, and measures of variation");
    print!("Enter data delimited by ; or , : ");
    io::stdout().flush().unwrap(); // print requires you to flush the buffer yourself, but not println

    let mut data: Vec<f32> = Vec::new();
    // Switch between
    io::stdin().read_line(&mut input).expect("Input Failed!");
    input.pop(); // remove newline char
    input.retain(|c| !c.is_whitespace()); // remove whitespace 
    let split_string_data: Vec<&str> = input.split(|c| c == ',' || c == ';').collect();
    for string in split_string_data {
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
    let mut input: String = String::new();
    println!("ARRANGEMENTS");
    println!("Calculates permutations and combinations for a given n and r");
    println!("(Enter only n for # of different permutations of n distinct objects)");
    print!("Enter n and r as \"n;r\" OR \"n,r\": ");
    io::stdout().flush().unwrap(); // print requires you to flush the buffer yourself, but not println

    io::stdin().read_line(&mut input).expect("Input Failed!");
    input.pop(); // remove newline char
    input.retain(|c| !c.is_whitespace()); // remove whitespace
    let split_string_data: Vec<&str> = input.split(|c| c == ',' || c == ';').collect();
    if split_string_data.len() == 2 {
        let n: f64 = split_string_data[0].parse::<f64>().unwrap();
        let r: f64 = split_string_data[1].parse::<f64>().unwrap();
        println!("Permutations: {}", arrangements::permutations_nr(n, r));
        println!("Combinations: {}", arrangements::combinations(n, r));
    }
    else if split_string_data.len() == 1 {
        let n: f64 = split_string_data[0].parse::<f64>().unwrap();
        println!("Permutations: {}", arrangements::permutations_n(n));
    }
    else {
        println!("Invalid input");
    }
}

fn probability_distribution_info() {
    let mut input: String = String::new();
    println!("PROBABILITY DISTRIBUTION INFO");
    println!("Gives info on mean, variance, and standard deviation of a probability distribution");
    print!("Enter x and its respective p as x;p and separate each pair with , :");
    io::stdout().flush().unwrap(); // print requires you to flush the buffer yourself, but not println

    io::stdin().read_line(&mut input).expect("Input Failed!");
    input.pop(); // remove newline char
    input.retain(|c| !c.is_whitespace()); // remove whitespace
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

fn binomial_probability_info() {
    loop {
        let mut continue_execution: bool = true;
	    println!("BINOMIAL PROBABILITY");
        let mut input: String = String::new();
        println!("0. Exit");
        println!("1. Binomial Probability Formula");
        println!("2. Binomial Probability Distribution Generator");
        
        print!("Enter Selection: "); io::stdout().flush().unwrap(); 

        io::stdin().read_line(&mut input).expect("Input Failed!"); input.pop();
        let trimmed_input: &str = input.trim();
        clear();
        match trimmed_input {
            "0" => { continue_execution = false },
            "1" => { // Binomial Probability Formula
                println!("Enter # of trials, # of successes, and probability of success\nin this order separated by , or ;");
                print!("Input: "); io::stdout().flush().unwrap();
                let mut input: String = String::new();
                io::stdin().read_line(&mut input).expect("Input Failed!"); input.pop();
                let bpf_trimmed_input: &str = input.trim();
                let bpf_split_data: Vec<&str> = bpf_trimmed_input.split(|c| c == ',' || c == ';').collect();
                let mut bpf_data: Vec<f32> = Vec::new();
                for string in bpf_split_data {
                    bpf_data.push(string.parse::<f32>().unwrap());
                    bpf_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
                }
                let result: f32 = binomial_probability::binomial_probability(bpf_data[0] as i32, bpf_data[1] as i32, bpf_data[2]);
                println!("Result: {}", result);
            },
            "2" => { // Binomial Probability Distribution Generator
                print!("Enter # of trials: "); io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Input Failed!"); input.pop(); input.retain(|c| !c.is_whitespace());
                let trials: i32 = input.parse::<i32>().unwrap();

                print!("Enter max # of successes: "); io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Input Failed!"); input.pop(); input.retain(|c| !c.is_whitespace());
                let max_successes: i32 = input.parse::<i32>().unwrap();

                print!("Enter probability of success: "); io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Input Failed!"); input.pop(); input.retain(|c| !c.is_whitespace());
                let probability_of_success: f32 = input.parse::<f32>().unwrap();

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

fn pause() {
    // pause
    io::stdin().read_line(&mut String::new()).unwrap();
    // clear console
    clear();
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
