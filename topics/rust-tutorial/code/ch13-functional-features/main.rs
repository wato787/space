fn main() {
    let add_one = |x: i32| x + 1;
    println!("add_one(1) = {}", add_one(1));

    let v = vec![1, 2, 3];
    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("v2 = {:?}", v2);

    let sum: u32 = Counter::new().take(5).sum();
    println!("sum = {sum}");
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}
