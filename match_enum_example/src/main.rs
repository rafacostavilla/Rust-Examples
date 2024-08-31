#[derive(Debug)]
enum usState {
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(usState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        // None => None,
        Some(i) => Some(i + 1),
        // _ => None,
        other => None,
    }
}

fn main (){
    let coin = Coin::Penny;
    println!("{}",value_in_cents(coin));
    value_in_cents(Coin::Quarter(usState::Alabama));

    println!("--------------------------------------");

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);
}