use is_prime::is_prime;

use super::common;

pub fn solve() {
  common::refresh_window();
  print!("{PROBLEM}");
  println!("{EXPLANATION}");
  // TODO: optimize this so it uses more space and less time.
  let mut i: u64 = 3;
  let mut rolling_sum: u64 = 2;
  while i < 2000000 {
    if is_prime(i.to_string().as_str()) {
      rolling_sum += i;
    }
    i += 2;
  }
  println!("    ↳  SOLUTION: {rolling_sum}  ↲\n\n");
  common::after_question();
}

const PROBLEM: &str = "
┌───────────────────────────────────────────────────────────────────────────────┐
│ The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17. Find the sum of all the │
│ primes below two million.                                                     │
└───────────────────────────────────────────────────────────────────────────────┘
";

const EXPLANATION: &str = 
"  This one just has a rolling sum of primes. Using the fast prime check from 
  question 7. NOTE: This one takes a while to run. Please be patient.
";