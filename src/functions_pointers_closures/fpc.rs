fn main() {
    let plus_one = |x: i32| x + 1;
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let plus_two = |x| {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };
    println!("{} + 2 = {}", a, plus_two(a));
}
