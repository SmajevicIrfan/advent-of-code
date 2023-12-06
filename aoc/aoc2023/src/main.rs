use std::env;

mod aoc2023_01;

pub fn main() {
	let args: Vec<String> = env::args().collect();

	let day = if args.len() > 1 {
		args[1].parse::<u32>().unwrap()
	} else {
		1
	};

	let part = if args.len() > 2 {
		args[2].parse::<u32>().unwrap()
	} else {
		1
	};

	match day {
		1 => aoc2023_01::run(part),
		_ => println!("Unknown day {}", day),
	}
}
