#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

//#[derive(Debug)]
//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn route(ip_type: IpAddrKind) {
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    //let home = IpAddr {
    //    kind: IpAddrKind::V4,
    //    address: String::from("127.0.0.1"),
    //};
    //let loopback = IpAddr {
    //    kind: IpAddrKind::V6,
    //    address: String::from("::1"),
    //};
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{home:?}");
    println!("{loopback:?}");
    println!("Hello, world!");
}
