fn main() {
  let char_variable: char = 'a';
  let char_variable2: char = 'b';
  let char_variable3: char = 'c';
  let char_variable4: char = 'd';

  println!("Char variable: {}", char_variable);
  println!("Char variable 2: {}", char_variable2);
  println!("Char variable 3: {}", char_variable3);
  println!("Char variable 4: {}", char_variable4);

  fn compare_chars(char1: char, char2: char) -> bool {
    char1 == char2
  }

  let result_compare_chars: bool = compare_chars(char_variable, char_variable2);
  println!("Result compare chars: {}", result_compare_chars);
}
