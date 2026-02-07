struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let point = Point { x: 0, y: 7 };

    match point {
        Point { x, y: 0 } => println!("on x-axis at {x}"),
        Point { x: 0, y } => println!("on y-axis at {y}"),
        Point { x, y } => println!("point: ({x}, {y})"),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Move { x, y } => println!("move to ({x}, {y})"),
        Message::Write(text) => println!("text: {text}"),
        Message::ChangeColor(r, g, b) => println!("color: {r}, {g}, {b}"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("first: {first}, last: {last}"),
    }
}
