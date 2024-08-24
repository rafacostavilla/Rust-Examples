fn main() {
    let mut my_string = String::from("Hello world!");
    let mut my_string_literal= "hello world";
    my_string_literal = "caruru";
    let index_or_word = first_word(&my_string);
    let index_or_word_2 = first_word(&my_string_literal);


    //my_string.clear();//This empties the String, making it equal to ""

    println!("{index_or_word}"); //Note that index_or_word should not print 
                                //any value after its String is empty
    println!("{index_or_word_2}");
    //my_string_literal.clear();
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i,&item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}