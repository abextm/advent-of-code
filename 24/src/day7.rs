#[aoc(part1=882304362421)]
fn part1(input: &str) -> impl std::fmt::Debug {
	input.lines()
		.map(|line| {
			let [result, line] = line.splitn(2, ": ").next_chunk().unwrap();
			let result = result.parse::<u64>().unwrap();
			let line = line.split(" ").map(|p| p.parse().unwrap()).collect::<Vec<u64>>();
			let bitmask = 1 << (line.len() - 1);
			if (0..bitmask).any(|bit| {
				let mut res = line[0];
				for (i, v) in line[1..].iter().enumerate() {
					if (1 << i) & bit == 0 {
						res += v
					} else {
						res *= v;
					}
				}
				res == result
			}) {
				result
			} else {
				0
			}
		}).sum::<u64>()
}

#[aoc(part1=3749, part2=11387)]
const EX: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

#[aoc(part2)]
fn part2(input: &str) -> impl std::fmt::Debug {
	input.lines()
		.map(|line| {
			let [result, line] = line.splitn(2, ": ").next_chunk().unwrap();
			let result = result.parse::<u64>().unwrap();
			let line = line.split(" ").map(|p| {
				(
					p.parse().unwrap(),
					10u64.pow(p.len() as u32),
				)
			}).collect::<Vec<(u64, u64)>>();
			let end = 3i32.pow(line.len() as u32 - 1);
			if (0..end).any(|mut bit| {
				let mut res = line[0].0;
				for (v, mul) in line[1..].iter() {
					match bit % 3 {
						0 => res += v,
						1 => res *= v,
						2 => {
							res = res * mul + v;
						},
						_ => unreachable!(),
					}
					bit /= 3;
				}
				res == result
			}) {
				result
			} else {
				0
			}
		}).sum::<u64>()
}