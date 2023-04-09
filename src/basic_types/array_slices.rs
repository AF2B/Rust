fn main() {
  let array: [i32; 3] = [1, 2, 3];
  let slice: &[i32] = &array[0..3];
  let array2 = [0; 3] // [0, 0, 0]

  println!("Array values: {:?}", array);
  println!("Slice values: {:?}", slice);

  for element in slice.iter() {
    println!("Element: {}", element);
  }

  for element in array.iter() {
    println!("Element: {}", element);
  }
}
