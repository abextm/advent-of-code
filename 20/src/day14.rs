#[aoc(day14, part1)]
fn day14_part1(input: &str) -> u64 {
	let t_mask = regex::Regex::new("^mask = ([X01]{36})$").unwrap();
	let t_assign = regex::Regex::new("^mem\\[([0-9]+)\\] = ([0-9]+)$").unwrap();
	let mut mem: Vec<u64> = Vec::new();
	let mut zero_mask = 0;
	let mut one_mask = 0;
	for line in input.trim().split("\n") {
		if let Some(c) = t_mask.captures(line) {
			zero_mask = 0;
			one_mask = 0;
			for (bit, val) in c[1].chars().rev().enumerate() {
				match val {
					'0' => zero_mask |= 1 << bit,
					'1' => one_mask |= 1 << bit,
					'X' => (),
					v => panic!("{}", v),
				}
			}
		} else if let Some(c) = t_assign.captures(line) {
			let ptr = c[1].parse().unwrap();
			if ptr >= mem.len() {
				mem.extend((mem.len()..=ptr).map(|_x| 0));
			}
			let val = c[2].parse::<u64>().unwrap();
			let val = one_mask | (val & !zero_mask);
			mem[ptr] = val;
		} else {
			panic!("{}", line);
		}
	}

	mem.iter().sum()
}

#[test]
fn testp1() {
	assert_eq!(day14_part1("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"), 101+64);
}

#[aoc(day14, part2)]
fn day14_part2(input: &str) -> u64 {
	let t_mask = regex::Regex::new("^mask = ([X01]{36})$").unwrap();
	let t_assign = regex::Regex::new("^mem\\[([0-9]+)\\] = ([0-9]+)$").unwrap();
	let mut mem = std::collections::HashMap::new();
	let mut one_mask = 0;
	let mut float_mask = 0;
	for line in input.trim().split("\n") {
		if let Some(c) = t_mask.captures(line) {
			one_mask = 0;
			float_mask = 0;
			for (bit, val) in c[1].chars().rev().enumerate() {
				match val {
					'0' => (),
					'1' => one_mask |= 1 << bit,
					'X' => float_mask |= 1 << bit,
					v => panic!("{}", v),
				}
			}
		} else if let Some(c) = t_assign.captures(line) {
			let val = c[2].parse::<u64>().unwrap();
			write_mask(&mut mem, c[1].parse::<u64>().unwrap() | one_mask, float_mask, 0, val);
		} else {
			panic!("{}", line);
		}
	}

	mem.values().sum()
}

#[test]
fn testp2() {
	assert_eq!(day14_part2("mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"), 208);
}

fn write_mask(mem: &mut std::collections::HashMap<u64, u64>, ptr: u64, float_mask: u64, float_shift: u64, value: u64) {
	if float_shift == 36 {
		println!("wr {} = {}", ptr, value);
		mem.insert(ptr, value);
		return
	}

	let bit: u64 = 1 << float_shift;
	if (float_mask & bit) != 0 {
		write_mask(mem, ptr | bit, float_mask, float_shift + 1, value);
		write_mask(mem, ptr & !bit, float_mask, float_shift + 1, value);
	} else {
		write_mask(mem, ptr, float_mask, float_shift + 1, value);
	}
}
