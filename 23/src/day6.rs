#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
	let mut input = input.lines()
		.map(|x| {
			x.split(" ")
				.filter(|b| b.len() > 0)
				.skip(1)
				.map(|x| x.parse().expect(x))
				.collect::<Vec<usize>>()
		});
	input.next().unwrap().iter().zip(input.next().unwrap().iter())
		.map(|(&time, &distance)| {
			let mut wins = 0;
			let mut speed = 0;
			for i in 1..time {
				speed += 1;
				let rem = time - i;
				if rem * speed > distance {
					wins += 1;
				}
			}

			wins
		}).reduce(|a, b| a * b).unwrap()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
	let mut input = input.lines()
		.map(|x| x.split(":").skip(1).next().unwrap().replace(" ", "").parse::<usize>().unwrap());
	let time = input.next().unwrap();
	let distance = input.next().unwrap();
	let mut wins = 0;
	let mut speed = 0;
	for i in 1..time {
		speed += 1;
		let rem = time - i;
		if rem * speed > distance {
			wins += 1;
		}
	}

	wins
}