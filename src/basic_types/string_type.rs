fn main() {
  let string_variable: String = String::from("Hello, world!");
  let string_variable2: String = String::from("Hello, André!");
  let string_variable3: String = String::from("Hello, Borba!");
  let string_variable4: String = String::from("Hello, =)!");

  println!("String variable: {}", string_variable);
  println!("String variable 2: {}", string_variable2);
  println!("String variable 3: {}", string_variable3);
  println!("String variable 4: {}", string_variable4);

  fn compare_strings(string1: String, string2: String) -> bool {
    string1 == string2
  }

  let result_compare_strings: bool = compare_strings(string_variable, string_variable2);
  println!("Result compare strings: {}", result_compare_strings);
}
