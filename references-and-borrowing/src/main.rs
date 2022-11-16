fn main() {
    length();
    mutable_references();

    println!("----");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        let len = calculate_length(&r1);

        println!("The length of '{}' is {}.", r1, len);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("r2 = {}", r2);
}

fn length() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    println!("s = {}", s);
    println!("&s = {}", *s);
    s.len()
}

fn mutable_references() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;

    println!("{}", r1);
    //    println!("{}", r2);
}
