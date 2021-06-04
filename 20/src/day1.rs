#[aoc(day1, part1)]
fn day1_part1(input: &str) -> i32 {
	let input = input
		.trim()
		.split("\n")
		.map(|str| str.parse::<i32>().expect("not an int"))
		.collect::<Vec<i32>>();

	let matches = input
		.iter()
		.filter(|&a| input.iter().any(|&b| a + b == 2020))
		.cloned()
		.collect::<Vec<i32>>();
	matches[0] * matches[1]
}

#[aoc(day1, part2)]
fn day1_part2(input: &str) -> i32 {
	let input = input
		.trim()
		.split("\n")
		.map(|str| str.parse::<i32>().expect("not an int"))
		.collect::<Vec<i32>>();

	for a in input.iter() {
		for b in input.iter() {
			for c in input.iter() {
				if a + b + c == 2020 {
					return a * b * c;
				}
			}
		}
	}
	panic!();
}
