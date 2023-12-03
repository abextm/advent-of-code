#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
	input.trim().lines()
		.map(|line| {
		let caps = regex::Regex::new(r"^[^0-9]*([0-9])(?:.*([0-9]))?[^0-9]*$").unwrap()
			.captures(line)
			.expect(line);
		let mut caps = caps.iter()
			.skip(1)
			.map(|x| x.unwrap().as_str().parse::<usize>().unwrap());
			
			let a = caps.next().unwrap();
			let b = caps.next().unwrap_or(a);
			a * 10 + b
		})
		.sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
	input.trim().lines()
		.map(|line| {
		let re = regex::Regex::new(r"([0-9])|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();
		let mut caps = OverlapCapturesIter{re: &re, haystack: line}
			.map(|cap| {
				let (index, matc) = cap.iter()
					.skip(1)
					.enumerate()
					.filter(|(_, x)| x.is_some())
					.next()
					.unwrap();
				
				if index == 0 {
					matc.unwrap().as_str().parse::<usize>().unwrap()
				} else {
					index
				}
			});

			let a = caps.next().unwrap();
			let b = caps.last().unwrap_or(a);
			a * 10 + b
		})
		.sum()
}

#[test]
fn test_p2() {
	assert_eq!(281, part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"));
	
	assert_eq!(21, part2("twone"));
}

struct OverlapCapturesIter<'r, 'h> {
	re: &'r regex::Regex,
	haystack: &'h str,
}

impl<'r, 'h> Iterator for OverlapCapturesIter<'r, 'h> {
	type Item = regex::Captures<'h>;

	fn next(&mut self) -> Option<regex::Captures<'h>> {
		let cap = self.re.captures(self.haystack);
		if let Some(c) = &cap {
			self.haystack = &self.haystack[c.get(0).unwrap().start() + 1..]
		}
		cap
	}
}