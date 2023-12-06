mod part1;
mod part2;

pub fn run(part: u32) {
	match part {
		1 => part1::run(),
		2 => part2::run(),
		_ => println!("Unknown part {}", part),
	}
}