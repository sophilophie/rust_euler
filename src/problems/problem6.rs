use super::common;

pub fn solve() {
  common::refresh_window();
  print!("{PROBLEM}");
  println!("{EXPLANATION}");
  let mut sum_of_squares: i32 = 0;
  for i in 1..=100 {
    sum_of_squares += i * i;
  }
  let mut square_of_sums: i32 = 0;
  for j in 1..=100 {
    square_of_sums += j;
  }
  let response: i32 = (square_of_sums * square_of_sums) - sum_of_squares;
  println!("    ↳  SOLUTION: {response}  ↲\n\n");
  common::after_question();
}

const PROBLEM: &str = "
┌───────────────────────────────────────────────────────────────────────────────┐
│ The sum of the squares of the first ten natural numbers is 385. The square of │
│ the sum of the first ten natural numbers is 3025. Hence the difference        │
│ between the sum of the squares of the first ten natural numbers and the       │
│ square of the sum is 2640. Find the difference between the sum of the squares │
│ of the first one hundred natural numbers and the square of the sum.           │
└───────────────────────────────────────────────────────────────────────────────┘
";

const EXPLANATION: &str =
"  Square of sums - sum of squares. Let's see if just brute focing does it.
  Yep it's just a couple of quick loops and some arithmatic.
";