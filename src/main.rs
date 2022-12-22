use std::io::{stdin, Error, stdout};
use std::io::Write;
use regex::Regex;
mod problems;

// TODO: write a macro once the switch statement gets out of hand :)

fn prompt(cursor: &str) -> String {
	let mut buffer: String = String::new();
	print!("{}", cursor);
	stdout().flush().expect("Error: Error flushing standard output!");
	stdin().read_line(&mut buffer).expect("Error: Error reading input!");
	buffer.trim().to_string()
}

fn main() -> Result<(), Error> {
	problems::common::refresh_window();
	println!("{GREETING}");
	let problem_regex: Regex = Regex::new(r"\d{1}").unwrap();
	Ok(loop {
		let input = prompt("▶ ");
		if input == "exit" {
			problems::common::refresh_window();
			break;
		} else if problem_regex.is_match(&input) {
			let input_num: u32 = input.parse().unwrap();
			match input_num {
				1 => problems::problem1::solve(),
				2 => problems::problem2::solve(),
				3 => problems::problem3::solve(),
				4 => problems::problem4::solve(),
				5 => problems::problem5::solve(),
				6 => problems::problem6::solve(),
				7 => problems::problem7::solve(),
				8 => problems::problem8::solve(),
				9 => problems::problem9::solve(),
				10 => problems::problem10::solve(),
				_ => println!("Problem not found!")
			}
		} else {
			println!("Invalid input");
		}
	})
}

const GREETING: &str = "
╔══════════════════════════════╗
║ Project Euler Rust Solutions ║
╚══════════════════════════════╝
Input the number of the problem. Type 'exit' to exit.
";