#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
	let id_re = regex::Regex::new("Game ([0-9]+):").unwrap();
	let cube_re = regex::Regex::new(" ([0-9]+) (red|green|blue)").unwrap();

	input.lines()
		.filter(|line| !cube_re.captures_iter(line)
			.any(|cap| {
				let val = cap[1].parse::<usize>().unwrap();
				val > match &cap[2] {
					"red" => 12,
					"green" => 13,
					"blue" => 14,
					v => panic!("{}", v),
				}
			}))
		.map(|line| id_re.captures(line).unwrap()[1].parse::<usize>().unwrap())
		.sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
	let cube_re = regex::Regex::new(" ([0-9]+) (red|green|blue)").unwrap();

	input.lines()
		.map(|line| {
			let mut max = [0, 0, 0];
			for cap in cube_re.captures_iter(line) {
				let val = cap[1].parse::<usize>().unwrap();
				let index = match &cap[2] {
					"red" => 0,
					"green" => 1,
					"blue" => 2,
					v => panic!("{}", v),
				};
				max[index] = max[index].max(val);
			}
			max.into_iter().reduce(|a, b| a * b).unwrap()
		})
		.sum()
}