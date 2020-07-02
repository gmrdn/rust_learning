fn main() {
    println!("Hello, world!");
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("called Message");
    }
}

#[test]
fn enum_values() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
