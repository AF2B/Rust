fn double_letters(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", double_letters("hello"));
    println!("{}", double_letters("world"));
    println!("{}", double_letters("mississippi"));
}
