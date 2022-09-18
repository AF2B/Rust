// All about rust language!

// 1. Variables

fn main() {
  // 1.1. Variables
  let x = 5; // immutable
  let mut y = 5; // mutable
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
  y = 6;
  println!("The value of y is: {}", y);

  // 1.2. Constants
  const MAX_POINTS: u32 = 100_000;
  const MIN_POINTS: u32 = 0;
  println!("The value of MAX_POINTS is: {}", MAX_POINTS);

  // 1.3. Shadowing
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("The value of x is: {}", x);
  let spaces = "   ";
  let spaces = spaces.len();
  println!("The value of spaces is: {}", spaces);

  // 1.4. Data types
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("The value of guess is: {}", guess);
  let x = 2.0; // f64
  let y: f32 = 3.0; // f32
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
  println!("The value of z is: {}", z);
  let five_hundred = tup.0;
  let six_point_four = tup.1;
  let one = tup.2;
  println!("The value of five_hundred is: {}", five_hundred);
  println!("The value of six_point_four is: {}", six_point_four);
  println!("The value of one is: {}", one);
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let second = a[1];
  println!("The value of first is: {}", first);
  println!("The value of second is: {}", second);

  // 1.5. Functions
  another_function(5, 6);
  let x = five();
  println!("The value of x is: {}", x);
  let y = plus_one(5);
  println!("The value of y is: {}", y);

  fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
  }

  fn greeting(name: &str) {
    println!("Hello {}!", name);
  }

  fn five() -> i32 {
    5
  } // return 5 as i32 type value

  fn plus_one(x: i32) -> i32 {
    x + 1
  }
  plus_one(5); // return 6 as i32 type value

  // 1.6. Control flow
  let number = 3;
  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }
  if number != 0 {
    println!("number was something other than zero");
  }
  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }
  let condition = true;
  let number = if condition { 5 } else { 6 };
  println!("The value of number is: {}", number);
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };
  println!("The value of result is: {}", result);

  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }

  let a = [10, 20, 30, 40, 50];
  for element in a.iter() {
    println!("the value is: {}", element);
  }

  for number in (1..4).rev() {
    println!("{}!", number);
  }

  // 1.7. Modules
  mod sound {
    pub mod instrument {
      pub fn clarinet() {
        // Function body code goes here
        println!("clarinet!");
      }
    }
  }
  sound::instrument::clarinet();

  mod codes {
    pub mod code1 {
      pub fn code1() {
        println!("code1!");
      }
    }
    pub mod code2 {
      pub fn code2() {
        println!("code2!");
      }
    }
  }
  use codes::code1;
  use codes::code2;
  code1::code1();
  code2::code2();

  // 1.8. External modules
  //...

  // 1.9. for
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  for i in a.iter() {
    if i == &3 {
      println!("3!");
    }else{
      println!("{}!", i);
    }
  }

  let mut i = 0;
  loop {
    println!("{}", i);
    i += 1;
    if i == 5 {
      break;
    }
  }

  for i in 0..a.len() {
    if a[i] == 3 {
      println!("3!");
    }else{
      println!("{}", a[i]);
    }
  }

  // 1.10. traditional for
  for i in 0..5 {
    println!("{}", i);
  }

  let number: i32 = 0;
  for i in 0..number {
    println!("{}", i);
  }

  // Explanation of &str with examples
  let s = "Hello, world!"; // s is of type &str
  let s = String::from("Hello, world!"); // s is of type String
  // &str is a reference to a string that is stored elsewhere
  // String is a string that is stored on the heap memory
  // &str is immutable
  // String is mutable

  // 1.11. String
  let mut s = String::from("foo");
  s.push_str("bar");
  println!("{}", s);
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
  println!("{}", s3);

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = s1 + "-" + &s2 + "-" + &s3;
  println!("{}", s);

  // 1.12. String format
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("{}", s);

  // 1.13. String index
  let s1 = String::from("hello");
  let h = s1[0];
  println!("{}", h);

  // 1.14. String slice
  let s = String::from("hello world");
  let hello = &s[0..5];
  let world = &s[6..11];
  println!("{}", hello); // hello
  println!("{}", world); // world

  // 1.15. Calculator in Rust!
  use std::io;
  fn main() {
    println!("Calculator");
    println!("Please input the expression.");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression)
      .expect("Failed to read line");
    println!("You typed: {}", expression);
  }

  // 1.16. Switch case
  fn main() {
    let number = 13;
    match number {
      1 => println!("one"),
      2 => println!("two"),
      3 => println!("three"),
      4 => println!("four"),
      5 => println!("five"),
      6 => println!("six"),
      7 => println!("seven"),
      8 => println!("eight"),
      9 => println!("nine"),
      10 => println!("ten"),
      _ => println!("something else"),
    }
  }
}