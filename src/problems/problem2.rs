use super::common;

fn get_fib(n: u32) -> u32 {
  match n {
    0 => 1,
    1 => 1,
    _ => get_fib(n - 1) + get_fib(n - 2)
  }
}

pub fn solve() {
  common::refresh_window();
  print!("{PROBLEM}");
  println!("{EXPLANATION}");
  print!("    ↳ SOLUTION: ");
  let mut counter: u32 = 1;
  while counter <= 10 {
    let fib: u32 = get_fib(counter);
    print!("{fib}, ");
    counter += 1;
  }
  println!("... ↲\n\n");
  common::after_question();
}

const PROBLEM: &str = "
┌───────────────────────────────────────────────────────────────────────────────┐
│ Each new term in the Fibonacci sequence is generated by adding the previous   │
│ two terms. By starting with 1 and 2, the first 10 terms will be:              │
└───────────────────────────────────────────────────────────────────────────────┘
";

const EXPLANATION: &str = 
"  Self explanatory. Just loop through a programmatic fibonacci sequence and
  print 10 times. Fibonacci sequence is determined by the Nth number of the
  sequence recursively.
";