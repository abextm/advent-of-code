#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
	let line_re = regex::Regex::new(r"Card +([0-9]+): ([0-9 ]+) \| ([0-9 ]+)").unwrap();
	input.lines()
		.map(|line| {
			let caps = line_re.captures(line).unwrap();
			let winning = caps[2].trim().split(" ")
				.filter(|x| x.len() > 0)
				.map(|x| x.parse().expect(x))
				.collect::<std::collections::HashSet<usize>>();
			let count = caps[3].trim().split(" ")
				.filter(|x| x.len() > 0)
				.map(|x| x.parse().expect(x))
				.filter(|x| winning.contains(x))
				.count();
			if count > 0 {
				2usize.pow((count - 1) as u32)
			} else {
				0
			}
		})
		.sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
	let line_re = regex::Regex::new(r"Card +([0-9]+): ([0-9 ]+) \| ([0-9 ]+)").unwrap();
	let mut qty = vec![1usize; 300];
	input.lines()
		.enumerate()
		.map(|(id, line)| {
			let caps = line_re.captures(line).unwrap();
			let winning = caps[2].trim().split(" ")
				.filter(|x| x.len() > 0)
				.map(|x| x.parse().expect(x))
				.collect::<std::collections::HashSet<usize>>();
			let count = caps[3].trim().split(" ")
				.filter(|x| x.len() > 0)
				.map(|x| x.parse().expect(x))
				.filter(|x| winning.contains(x))
				.count();
			let winnings = qty[id];
			for qt in &mut qty[(id + 1)..(id + 1 + count)] {
				*qt += winnings;
			}
			qty[id]
		})
		.sum()
}