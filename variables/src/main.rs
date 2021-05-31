fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The len of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x,y is: {} {}", x, y);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c, z, heart_eyed_cat is: {} {} {}", c, z, heart_eyed_cat);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x,y,z is: {} {} {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(
        "five_hundred, six_point_four, one is: {} {} {}",
        five_hundred, six_point_four, one
    );

    let a = [1, 2, 3, 4, 5];
    let index = 2;
    let element = a[index];
    println!("The value of element is: {}", element);
}
