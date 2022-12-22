use super::common;

pub fn solve() {
  common::refresh_window();
  print!("{PROBLEM}");
  println!("{EXPLANATION}");
  let mut palindrome: i32 = 0;
  let mut answer: String = String::new();
  let mut i: i32 = 100;
  let mut j: i32;
  while i < 1000 {
    j = 100;
    'inner: while j < 1000 {
      let str: String = (i * j).to_string();
      let rev_str: String = str.chars().rev().collect::<String>();
      if rev_str == str && (i * j) > palindrome {
        palindrome = i * j;
        answer = format!("{i} × {j}");
        i = j;
        break 'inner;
      }
      j += 1;
    }
    i += 1;
  }
  println!("    ↳  SOLUTION: {palindrome} ({answer})  ↲\n\n");
  common::after_question();
}

const PROBLEM: &str = "
┌───────────────────────────────────────────────────────────────────────────────┐
│ A palindromic number reads the same both ways. The largest palindrome made    │
│ from the product of two 2-digit numbers is 9009 = 91 × 99. Find the largest   │
│ palindrome made from the product of two 3-digit numbers.                      │
└───────────────────────────────────────────────────────────────────────────────┘
";

const EXPLANATION: &str =
"  For this one, we do a loop within a loop that will push the its value to the
  outer loop when it finds a palindrome. As long as we check to see if the
  product is bigger than the previous palindrome, we can keep skipping for
  the computational savings while finding the genuine largest palindrome.
";