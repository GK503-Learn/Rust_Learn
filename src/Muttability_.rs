const MAX_POINTS: u32 = 100_000; // this is a constant and is immutable - type is required

fn main() {
  // let x = 5; // Immutable and cannot be changed after set
  let mut x = 5; // Muttable variable so it can be changed leter on
  println!("x is {}", x); // Curly brace is a placeholder for x
  x = 6;
  println!("x is {}", x);

  // Shadowing
  let y: i64 = 5;
  println!("y is {}", y);

  let y: i64 = y + 1;
  println!("y is {}", y)
}