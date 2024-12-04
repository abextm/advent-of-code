#[aoc(part1=161289189)]
fn solve(input: &str) -> impl std::fmt::Debug {
	let re = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
	re.captures_iter(input)
		.map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap() * c.get(2).unwrap().as_str().parse::<i32>().unwrap())
		.sum::<i32>()
}

#[aoc(part2=83595109)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let re = regex::Regex::new(r"(?:mul\(([0-9]+),([0-9]+)\))|(?:(do|don't)\(\))").unwrap();
	let mut en = true;
	let mut sum = 0i32;
	for c in re.captures_iter(input) {
		if let Some(v) = c.get(1) {
			if en {
				sum += c.get(1).unwrap().as_str().parse::<i32>().unwrap() * c.get(2).unwrap().as_str().parse::<i32>().unwrap()
			}
		} else if let Some(v) = c.get(3) {
			en = v.as_str() == "do";
		}
	}

	sum
}

#[aoc(part2=48)]
const EX2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mu\
\
\
l(8,5))";