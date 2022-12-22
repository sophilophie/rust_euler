use super::common;

pub fn solve() {
  common::refresh_window();
  print!("{PROBLEM}");
  println!("{EXPLANATION}");
  let mut m: i64 = 2;
  let mut n: i64 = 1;
  let mut a;
  let mut b;
  let mut c;
  let answer;
  'OUTER: loop {
    'INNER: loop {
      a = m.pow(2) - n.pow(2);
      b = 2 * m * n;
      c = m.pow(2) + n.pow(2);
      if a + b + c == 1000 {
        answer = a * b * c;
        break 'OUTER;
      }
      if a + b + c > 1000 {
        break 'INNER;
      }
      m +=1;
    }
    n +=1;
    m = n + 1;
  }
  println!("    ↳  SOLUTION: {answer}  ↲\n\n");
  common::after_question();
}

const PROBLEM: &str = "
┌───────────────────────────────────────────────────────────────────────────────┐
│ A Pythagorean triplet is a set of three natural numbers, a < b < c, for which │
│ a^2 + b^2 = c^2 For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2. There exists      │
│ exactly one Pythagorean triplet for which a + b + c = 1000. Find the product  │
│ abc.                                                                          │
└───────────────────────────────────────────────────────────────────────────────┘
";

const EXPLANATION: &str = 
"  This one we just brute force using Euclid's formula for finding pythagorean
  triples. Once they get more than 1000 we know that M is not going to work
  with any value N so we break the inner loop and try with an N that is one
  higher. Once we find that magic set we return the multiplied product.
";
