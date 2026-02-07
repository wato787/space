fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s2 = gives_ownership();
    let s3 = takes_and_gives_back(s2);

    let len = calculate_length(&s3);
    println!("length of '{}' is {}", s3, len);

    let mut s4 = String::from("hello");
    change(&mut s4);
    let slice = &s4[0..2];
    println!("slice: {slice}");
}

fn takes_ownership(s: String) {
    println!("owned: {s}");
}

fn makes_copy(x: i32) {
    println!("copied: {x}");
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}
