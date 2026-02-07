#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("message received");
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?} {:?}", home, loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let value = Some(3);
    if let Some(v) = value {
        println!("value: {v}");
    }
}
