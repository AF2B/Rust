fn main() {
  let is_raining = true;

  match is_raining {
    true => println!("It's raining"),
    false => println!("It's not raining"),
  }

  let condition: bool = false;

  if condition {
    println!("Condition is true");
  } else {
    println!("Condition is false");
  }

  fn change_condition(condition: bool) -> bool {
    !condition
  }

  let new_condition: bool = change_condition(condition);

  if new_condition {
    println!("New condition is true");
  } else {
    println!("New condition is false");
  }
}
