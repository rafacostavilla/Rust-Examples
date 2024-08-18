fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _) = tup;

    println!("The value of y is: {y}");

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;


    println!("Tuple is: {a}, {b}, {c}");

    let mut new_tuple = (1, 2);
    new_tuple.0 = 0;
    new_tuple.1 += 5;

    let a = new_tuple.0;
    let b = new_tuple.1;

    println!("{a} and {b}")

}
