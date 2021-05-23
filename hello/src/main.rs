fn main() {
    let value = 1;
    let mut value2 = 1;
    if (value == 1) {
        value2 = 2;
    }
    const MAX_POINT: u32 = 100_000;
    println!("{} {} {}", value, value2, MAX_POINT);
}
