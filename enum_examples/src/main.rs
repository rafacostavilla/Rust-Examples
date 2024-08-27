enum IpAddrKind{
    V4(String),
    V6(String),
}

fn main() {
    let _home = IpAddrKind::V4(String::from("127.0.0.1"));

    let _loopback = IpAddrKind::V6(String::from("::1"));
}
