use std::io;

fn main() {
    println!("Welcome to temperature converter.");
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

    // Formulas
    // F to C => C = (F - 32) / 1,8
    // C to F => F = C * 1,8 + 32

    match picked_option {
        1 => println!("Fahrenheit to celsius"),
        2 => println!("celsius to fahrenheit"),
        _ => {
            // we'll never reach this
            // we enforce the user to pick an available option
            // in the loop above
            println!("option not supported");
        }
    }
}
