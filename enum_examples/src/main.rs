enum IpAddrKind{
    V4,
    V6,
}

struct  IpAddr {
    _kind: IpAddrKind,
    _address: String,
}

fn main() {
    let _home = IpAddr{
        _kind: IpAddrKind::V4,
        _address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr{
        _kind: IpAddrKind::V6,
        _address: String::from("::1"),
    };
}
