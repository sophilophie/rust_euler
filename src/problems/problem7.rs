use super::common;
use is_prime::is_prime;

pub fn solve() {
  common::refresh_window();
  println!("{PROBLEM}");
  println!("{EXPLANATION}");
  let mut counter: i64 = 6;
  let mut test_num: i64 = 13;
  let mut prime: i64 = 13;
  while counter <= 10001 {
    if is_prime(test_num.to_string().as_str()) {
      counter += 1;
      prime = test_num;
    }
    test_num += 2;
  }
  println!("    ↳  SOLUTION: {prime}  ↲\n\n");
  common::after_question();
}

const PROBLEM: &str = "
┌───────────────────────────────────────────────────────────────────────────────┐
│ By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see    │
│ that the 6th prime is 13. What is the 10 001st prime number?                  │
└───────────────────────────────────────────────────────────────────────────────┘
";

const EXPLANATION: &str =
"  This one just loops checking if all numbers are prime until the 10001th is 
  found. I reached for the is_prime crate because it uses a fast primality
  check. (Miller-Rabin) also skipped all even numbers because primes are all
  odd except for two.
";