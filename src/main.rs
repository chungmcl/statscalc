use std::io;
use std::io::Write;

mod general_funcs;
mod central_tendency;
mod measures_of_variation;
mod measures_of_position;
mod arrangements;

fn main() {
    loop {
        let mut continue_execution: bool = true;
	    println!("statscalc");
        let mut input: String = String::new();
        println!("0. Exit");
        println!("1. Data Set Info");
        println!("2. Arrangments");
        
        print!("Enter Selection: ");
        io::stdout().flush().unwrap(); // print requires you to flush the buffer yourself, but not println

        // Switch between
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.pop();
                let trimmed_input: &str = input.trim();
                clear();
                match trimmed_input {
                    "0" => { continue_execution = false },
                    "1" => data_set_info(),
                    "2" => arrangements_info(),
                    _ => {
                        println!("Invalid selection. Please try again.");
                    }
                }
            },
            Err(e) => println!("Error: {}", e)
        }
        if continue_execution { pause(); }
        else { break; }
    }
}

fn data_set_info() {
    let mut input: String = String::new();
    println!("DATA SET INFO");
    println!("Calculates central tendencies, measures of position, and measures of variation");
    print!("Enter data delimited by semicolon or comma: ");
    io::stdout().flush().unwrap(); // print requires you to flush the buffer yourself, but not println

    let mut data: Vec<f32> = Vec::new();
    // Switch between
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop(); // remove newline char
            input.retain(|c| !c.is_whitespace()); // remove whitespace 
            let split_string_data: Vec<&str> = input.split(|c| c == ',' || c == ';' || c == ' ').collect();
            for string in split_string_data {
                data.push(string.parse::<f32>().unwrap());
                data.sort_by(|a, b| a.partial_cmp(b).unwrap());
            }
        },
        Err(e) => println!("Error: {}", e)
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
    print!("Enter n and r as \"n;r\" OR \"n,r\" OR \"n r\": ");
    io::stdout().flush().unwrap(); // print requires you to flush the buffer yourself, but not println

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop(); // remove newline char
            input.retain(|c| !c.is_whitespace()); // remove whitespace
            let split_string_data: Vec<&str> = input.split(|c| c == ',' || c == ';' || c == ' ').collect();
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
        },
        Err(e) => println!("Error: {}", e)
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
