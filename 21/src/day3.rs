#[aoc(day3, part1)]
fn day3_part1(input: &str) -> u32 {
	let num_bits = input.find("\n").unwrap();
	let input = parse_input(input);

	let mut o = 0;
	for bit in 0..num_bits {
		o |= (max_bit(&input, bit) as u32) << bit;
	}
	o * (o ^ ((1<<num_bits) - 1))
}

fn parse_input(input: &str) -> Vec<u32> {
	input.trim().split("\n")
		.map(|x| u32::from_str_radix(x, 2).unwrap())
		.collect()
}

fn max_bit(input: &[u32], bit: usize) -> bool {
	let ones: usize = input.iter()
		.map(|&x| (x as usize >> bit) & 1)
		.sum();
	ones >= (input.len() - ones)
}

#[aoc(day3, part2)]
fn day3_part2(input: &str) -> u32 {
	let num_bits = input.find("\n").unwrap();
	let input = parse_input(input);
	filter(input.clone(), num_bits, false) * filter(input, num_bits, true)
}

fn filter(mut report: Vec<u32>, num_bits: usize, invert: bool) -> u32 {
	for bit in (0..num_bits).rev() {
		let target = (max_bit(&report, bit) ^ invert) as u32;
		report.retain(|&x| (x >> bit) & 1 == target);
		if report.len() == 1 {
			return report[0];
		}
	}
	panic!();
}