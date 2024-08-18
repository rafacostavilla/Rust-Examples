fn main() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    let first = array[0];
    let second = array[1];

    println!("First: {first} and Second: {second}");

    let three_array = [3; 5];

    let last_three = three_array[4];
    println!("{last_three}");
}
