fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _) = tup;

    println!("The value of y is: {y}");
}
