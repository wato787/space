fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    let result = largest(&nums);
    println!("largest: {result}");

    let article = Article {
        headline: String::from("Rust 1.80 Released"),
        author: String::from("Ferris"),
    };
    println!("summary: {}", article.summarize());

    let s1 = String::from("abcd");
    let s2 = "xyz";
    let res = longest(s1.as_str(), s2);
    println!("longest: {res}");
}
