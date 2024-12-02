use std::collections::HashSet;

#[aoc(part1=369,part2=428)]
fn solve(input: &str, part1: bool) -> impl std::fmt::Debug {
	input.lines().filter(|x| {
		let arr = x.split(" ").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
		if test(|| arr.iter()) {
			return true;
		}

		if part1 {
			return false;
		}

		for ii in 0..arr.len() {
			if test(|| arr.iter().enumerate().filter(|(i, _v)| *i != ii).map(|(_i, v)| v)) {
				return true;
			}
		}

		false
	}).count()
}

fn test<'a, I: Iterator<Item=&'a i32>>(iter: impl Fn() -> I) -> bool {
	let all_same = iter().zip(iter().skip(1))
		.map(|(a, b)| a.cmp(b)).collect::<HashSet<_>>().len() == 1;
	if !all_same {
		return false;
	}
	iter().zip(iter().skip(1)).all(|(a, b)| {
		let v = (a - b).abs();
		v >= 1 && v <= 3
	})
}