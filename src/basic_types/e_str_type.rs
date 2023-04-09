fn main() {
  let slice_string: &str = "Hello, world!";

  println!("Slice string: {}", slice_string);

  fn iterate_string(slice_string: &str) {
    for character in slice_string.chars() {
      println!("{}", character);
    }
  }

  iterate_string(slice_string);
}
