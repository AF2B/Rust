fn main() {
  let array: [i32; 3] = [1, 2, 3];
  let slice: &[i32] = &array[0..3];

  println!("Array values: {:?}", array);
  println!("Slice values: {:?}", slice);

  for element in slice.iter() {
    if element == &[0] {
      array[0] = 0;
      println!("Array values: {:?}", array);
    }
  }
}
