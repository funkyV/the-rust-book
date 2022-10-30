use std::io;

fn main() {
    loop {
        println!("Welcome to temperature converter.");
        println!("To quit, press Ctrl+C");
        println!("Please choose a conversion type:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        let available_options = [1, 2];
        let picked_option: u32 = loop {
            let mut option = String::new();

            io::stdin()
                .read_line(&mut option)
                .expect("Failed to read line");

            match option.trim().parse() {
                Ok(num) => {
                    if !available_options.contains(&num) {
                        println!("Option {num} is not available.");
                        continue;
                    }

                    break num;
                }
                Err(_) => {
                    println!("Wrong input. Please type the option number.");
                    continue;
                }
            };
        };

        println!("You picked: {picked_option}");

        match picked_option {
            1 => println!("Please enter # of Fahrenheit degrees."),
            2 => println!("Please enter # of Celsius degrees."),
            _ => {
                // we'll never reach this
                // we enforce the user to pick an available option
                // in the loop above
                println!("option not supported");
            }
        }

        let num_of_degrees: f32 = loop {
            let mut degrees_input = String::new();
            io::stdin()
                .read_line(&mut degrees_input)
                .expect("Failed to read line.");

            match degrees_input.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Please enter a number.");
                    continue;
                }
            }
        };

        match picked_option {
            1 => {
                let result = fahrenheit_to_celsius(num_of_degrees);
                println!("{num_of_degrees}°F is equal to {result}°C ");
            }
            2 => {
                let result = celsius_to_fahrenheit(num_of_degrees);
                println!("{num_of_degrees}°C is equal to {result}°F ");
            }
            _ => println!("Will not reach this"),
        }
        println!("-- done --");
        println!();
    }
}

// Formulas
// F to C => C = (F - 32) / 1,8
// C to F => F = C * 1,8 + 32
fn fahrenheit_to_celsius(x: f32) -> f32 {
    (x - 32.0) / 1.8
}

fn celsius_to_fahrenheit(x: f32) -> f32 {
    x * 1.8 + 32.0
}
