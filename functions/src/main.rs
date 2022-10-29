fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // statement vs expression
    // let x = 5; - statement
    // print_labeled_measurement(5, 'h'); - statement
    // the function call above would be an expression
    // if print_labeled_measurement would've return a value

    let y = {
        let x = 3;
        // x + 1; - semicolon transforms x + 1 expression to statement
        x + 1 // expresion
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");
    
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}