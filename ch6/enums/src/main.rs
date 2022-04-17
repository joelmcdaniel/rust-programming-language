/* enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {} */

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        use Message::*;
        match &self {
            Quit => println!("Message is quitting..."),
            Write(s) => println!("Message is '{}'", s),
            _ => (),
        }
    }
}

fn main() {
    /* let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6); */

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let m = Message::Move { x: 1, y: 2 };
    m.call();

    let m = Message::Quit;
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("absent_number: {:?}", absent_number);
}
