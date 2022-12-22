pub fn after_question() {
  println!("Input the number of another problem. Type 'exit' to exit.");
}

pub fn refresh_window() {
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}