#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

impl User {
    fn new(username: &str, email: &str) -> User {
        User {
            username: username.to_string(),
            email: email.to_string(),
            active: true,
            sign_in_count: 1,
        }
    }

    fn display(&self) -> String {
        format!("{} <{}>", self.username, self.email)
    }
}

fn main() {
    let mut user1 = User::new("alice", "alice@example.com");
    user1.email = String::from("alice@rust.dev");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", user1);
    println!("{}", user1.display());
    println!("color: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);
}
