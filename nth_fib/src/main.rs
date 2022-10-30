use std::io;

fn main() {
    println!("Hello. This will compute the n-th fibonnaci number");
    println!("Please enter a value for n");

    loop {
        let mut n_input = String::new();

        io::stdin()
            .read_line(&mut n_input)
            .expect("Failed to read line");

        match n_input.trim().parse::<i32>() {
            Ok(n) => {
                if n == 0 {
                    println!("n must be greater than 0.");
                    continue;
                }

                let nth_fib = compute_nth_fib_number(n);
                println!("The {n}th fib number is: {nth_fib}");
            }
            Err(_) => println!("Please enter an integer"),
        };
    }
}

fn compute_nth_fib_number(n: i32) -> i64 {
    // 0, 1, 1, 2, 3, 5, 8, 13 ...

    if n == 1 {
        println!("Previous fib numbers: 0");
        return 0;
    } else if n == 2 {
        println!("Previous fib numbers: 0, 1");
        return 1;
    }

    let mut current_index: usize = 3;
    let mut previous_fib = 0;
    let mut current_fib = 1;
    let mut next_fib: i64 = previous_fib + current_fib;

    print!("Previous fib numbers: 0, 1, ");

    while current_index != n as usize {
        if current_index == n as usize - 1 {
            print!("{next_fib}\n");
        } else {
            print!("{next_fib}, ");
        }

        previous_fib = current_fib;
        current_fib = next_fib;
        next_fib = previous_fib + current_fib;
        current_index += 1;
    }

    next_fib
}
