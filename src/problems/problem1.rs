use super::common;

pub fn solve() {
  common::refresh_window();
  print!("{PROBLEM}");
  println!("{EXPLANATION}");
  let mut sum = 0;
  for num in 1..1000 {
    if num % 3 == 0 || num % 5 == 0 {
      sum += num;
    }
  }
  println!("    ↳ SOLUTION: {sum} ↲\n\n");
  common::after_question();
}

const PROBLEM: &str = "
┌───────────────────────────────────────────────────────────────────────────────┐
│ If we list all the natural numbers below 10 that are multiples of 3 or 5, we  │
│ get 3, 5, 6 and 9. The sum of these multiples is 23. Find the sum of all the  │
│ multiples of 3 or 5 below 1000.                                               │
└───────────────────────────────────────────────────────────────────────────────┘
";

const EXPLANATION: &str = 
"  Okay, this looks like this is just a sum of multiples. All we need to do here
  is check if N % 3 == 0 || N % 5 == 0 and add to a rolling sum until we hit
  1000. Just going to write a simple loop and print the answer.
";