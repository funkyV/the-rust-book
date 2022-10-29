fn main() {
    // shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    println!("The value of x is: {x}");

    // tuples
    // fixed length
    // elements can have different types

    let tup = (500, 6.4, 1);

    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("types? {} {} {}", tup.0, tup.1, tup.2);
    
    println!("The value of x is {x}, y is: {y}, z is: {z}");

    // arrays
    // fixed length
    // elements have same type
    let a = [1, 2, 3, 4, 5];

    // specify type and len
    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    // specify element and num of occurences/len
    // let a: [3; 5]; => [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];

    println!("first: {first}, second: {second}");   
}
