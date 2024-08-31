fn main() {
    let config_max:Option<u8> = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }

    //The above case is similar to the below one. Also, the last commented lines could be used for a default case
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }//else{
     // ();
    // }
}
