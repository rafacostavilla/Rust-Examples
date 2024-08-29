fn main() {
    //Compiler can infer type for Some variant
    let some_number = Some(5);
    let some_char = Some('e');
    //We need make the type explicit for None variant
    let absent_number: Option<i32>= None;
}
