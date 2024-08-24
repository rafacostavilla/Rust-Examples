fn main() {
    println!("Hello, world!");

    another_function(five(), 'm');
}

fn another_function(x: i32, unity_label: char) {
    println!("Another function value: {x}{unity_label}");
}
/*
Multiline
Comment
 */
fn five() -> i32 {
    5
}
