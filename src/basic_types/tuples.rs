fn first_way_to_create_tuple() -> (i32, f64, u8) {
  (500, 6.4, 1)
}

fn second_way_to_create_tuple() -> (i32, f64, u8) {
  let tuple_variable: (i32, f64, u8) = (500, 6.4, 1);
  tuple_variable
}

fn read_tuple(tuple_param: (i32, f64, u8)) -> () {
  let tuple: (i32, f64, u8) = tuple_param;
  let array: [i32; 3] = [tuple.0, tuple.1 as i32, tuple.2 as i32];

  for element in array.iter() {
    println!("Element: {}", element);
  }
}

fn main() {
  let first_tuple: (i32, f64, u8) = first_way_to_create_tuple();
  let second_tuple: (i32, f64, u8) = second_way_to_create_tuple();

  println!("First tuple: {:?}", first_tuple);
  println!("Second tuple: {:?}", second_tuple);
  println!("First tuple.0: {}", first_tuple.0);
  println!("First tuple.1: {}", first_tuple.1);
  println!("First tuple.2: {}", first_tuple.2);

  let tuple_variable: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tuple_variable;
  println!("x: {}", x);
  println!("y: {}", y);
  println!("z: {}", z);

  read_tuple((500, 6.4, 1));
}
