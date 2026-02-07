use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);

    let third = &v[2];
    println!("third: {third}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("string: {s}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
