pub fn run() {
	let input = include_str!("input_01.txt");
	let sum = input
		.lines()
		.map(|line| process_line(line))
		.sum::<u32>();
	
	println!("{}", sum);
}

fn process_line(line: &str) -> u32 {
	let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap() as u8 - b'0';
	let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap() as u8 - b'0';
	return (first_digit as u32) * 10 + (last_digit as u32);
}