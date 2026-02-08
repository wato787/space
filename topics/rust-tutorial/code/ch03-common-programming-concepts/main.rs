fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let mut x = 5;
    x += 1;

    const MAX_POINTS: u32 = 100_000;

    let y = {
        let z = x + 1;
        z * 2
    };

    let sum = add(3, 4);

    println!("x = {x}, y = {y}, sum = {sum}, max = {MAX_POINTS}");

    let tup = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("tuple: {a}, {b}, {c}");

    let arr = [1, 2, 3, 4, 5];
    for element in arr {
        println!("value: {element}");
    }

    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("LIFTOFF!!!");
}
