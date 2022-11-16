fn main() {
    let s = "hello";
    println!("literal (type might be str? im not sure) s = {}", s);

    let mut s = String::from("hello");
    println!("String s = {}", s);

    s.push_str(" World!");
    println!("{}", s);

    let s1 = String::from("Hi, i am s1");
    let s2 = s1;

    println!("s2 = {}", s2);
    // print!("s1 = {}", s1);
    //                   ^^ value borrowed here after move

    // with clone
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scop
