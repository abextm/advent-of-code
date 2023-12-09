#[aoc(day9, part1)]
fn part1(input: &str) -> isize {
	input.lines()
		.map(|line| {
			let mut line = line.split(" ")
				.map(|x| x.parse::<isize>().unwrap())
				.collect::<Vec<_>>();
			let mut stack = Vec::new();
			while !line.iter().all(|&x| x == 0) {
				stack.push(*line.last().unwrap());
				line = line.iter().skip(1).zip(line.iter())
					.map(|(a, b)| (a - b))
					.collect::<Vec<isize>>();
			}

			stack.iter().sum::<isize>()
		})
		.sum()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> isize {
	input.lines()
		.map(|line| {
			let mut line = line.split(" ")
				.map(|x| x.parse::<isize>().unwrap())
				.collect::<Vec<_>>();
			let mut stack = Vec::new();
			while !line.iter().all(|&x| x == 0) {
				stack.push(*line.first().unwrap());
				line = line.iter().skip(1).zip(line.iter())
					.map(|(a, b)| (a - b))
					.collect::<Vec<isize>>();
			}

			let v = stack.into_iter().rev().fold(0, |a, b| b - a);
			println!("{}", v);
			v
		})
		.sum()
}

#[test]
fn test_p2() {
	assert_eq!(2, part2("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"));
}