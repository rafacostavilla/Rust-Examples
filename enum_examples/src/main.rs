enum IpAddrKind{
    V4,
    V6,
}

fn route(_ip_kind: IpAddrKind){}

fn main() {
    let four = IpAddrKind::V4;
    route(four);
    let six = IpAddrKind::V6;
    route(six);
}
