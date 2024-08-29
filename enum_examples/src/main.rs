enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self){
        match self {
            Message::Write(value) => {
                println!("{value}");
            }
            _ => (),
        }
    }   
}

fn main() {
    let _home = IpAddrKind::V4(127, 0, 0, 1);

    let _loopback = IpAddrKind::V6(String::from("::1"));

    let message = Message::Write(String::from("hello"));
    message.call();
    
}
