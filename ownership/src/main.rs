fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();
    println!("{}", s2);
    // println!("{}", s1);
    println!("{}", s3);

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
