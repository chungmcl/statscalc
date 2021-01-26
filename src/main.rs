use std::io;
use std::io::Write;

mod general_funcs;
mod central_tendency;
mod measures_of_variation;
mod measures_of_position;

fn main() {
    loop {
        let mut input: String = String::new();
	println!("statscalc");
        print!("Enter data delimited by semicolon: ");
        io::stdout().flush().unwrap(); // print requires you to flush the buffer yourself, but not println
    
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

        let central_tendencies: (f32, f32, f32) = central_tendency::get_all(&data);
        let measures_of_variation: (f32, f32) = measures_of_variation::get_all(&data);
        let quartiles: (f32, f32, f32, f32) = measures_of_position::get_quartiles(&data);

        println!("Population StdDev: {}\nSample StdDev: {}", measures_of_variation.0, measures_of_variation.1);
        println!("Mean: {}\nMedian: {}\nMode: {}", central_tendencies.0, central_tendencies.1, central_tendencies.2);
        println!("Quartiles: 1st = {}, 2nd = {}, 3rd = {}, IQR = {}", quartiles.0, quartiles.1, quartiles.2, quartiles.3);

        general_funcs::pause();
    }
}
