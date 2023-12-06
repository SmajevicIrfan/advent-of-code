use aho_corasick::AhoCorasick;

pub fn run() {
	let digits = ["one", "1", "two", "2", "three", "3", "four", "4", "five", "5",
		"six", "6", "seven", "7", "eight", "8", "nine", "9"];
	let ac = AhoCorasick::new(&digits).unwrap();

	let input = include_str!("input_01.txt");
	let sum = input
		.lines()
		.map(|line| process_line(line, &ac))
		.sum::<u32>();
	
	println!("{}", sum);
}

fn process_line(line: &str, ac: &AhoCorasick) -> u32 {
	let matches: Vec<_> = ac.find_overlapping_iter(line).collect();
	let first_digit = matches.first().unwrap().pattern().as_u32() / 2 + 1;
	let last_digit = matches.last().unwrap().pattern().as_u32() / 2 + 1;

	return first_digit * 10 + last_digit;
}
