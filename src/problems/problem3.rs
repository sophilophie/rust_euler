use super::common;

pub fn solve() {
  common::refresh_window();
  print!("{PROBLEM}");
  println!("{EXPLANATION}");
  let mut start_num: i64 = 600851475143;
  let mut test_num: i64 = 2;
  while test_num < start_num {
    if start_num % test_num == 0 {
      start_num = start_num / test_num;
      test_num = 2;
    } else {
      test_num += 1;
    }
  }
  println!("    ↳  SOLUTION: {test_num}  ↲\n\n");
  common::after_question();
}

const PROBLEM: &str = "
┌───────────────────────────────────────────────────────────────────────────────┐
│ The prime factors of 13195 are 5, 7, 13 and 29. What is the largest prime     │
│ factor of the number 600851475143 ?                                           │
└───────────────────────────────────────────────────────────────────────────────┘
";

const EXPLANATION: &str = 
"  This one gets computationally expensive if done with a looped prime check.
  Not to worry though, we can just keep testing starting from two if it has no
  remainder with the original number and this will guarantee that if division
  is performed it will be prime. So we just keep dividing this beast of a
  number with any of it's smaller factors and starting over in case of repeated
  prime factors. By the end, this number won't be divisible by anything smaller
  than its running quotient, and will therefore be the largest prime factor!
  Since it keeps shrinking via the division step, so will the number of times
  it will need to run this loop.
";