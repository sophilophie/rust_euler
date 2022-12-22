use super::common;

pub fn solve() {
  common::refresh_window();
  print!("{PROBLEM}");
  println!("{EXPLANATION}");
  let nums: [i32; 10] = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
  let mut test_num: i32 = 2520;
  loop {
    let mut all_divisible = true;
    for i in nums {
      if test_num % i != 0 {
        all_divisible = false;
      }
    }
    if all_divisible {
      break;
    }
    test_num += 2520;
  }
  println!("    ↳  SOLUTION: {test_num}  ↲\n\n");
  common::after_question();
}

const PROBLEM: &str = "
┌───────────────────────────────────────────────────────────────────────────────┐
│ 2520 is the smallest number that can be divided by each of the numbers from 1 │
│ to 10 without any remainder. What is the smallest positive number that is     │
│ evenly divisible by all of the numbers from 1 to 20?                          │
└───────────────────────────────────────────────────────────────────────────────┘
";

const EXPLANATION: &str =
"  So this has a workaround. 2520 is the smallest number divisible by 1 through
  ten, so we can be sure that the smallest number divisible evenly by 1 to 20
  is a multiple of this number. Then we only have to check numbers 11 through
  20 in a loop for divisibility incrementing by 2520 each go around.
";