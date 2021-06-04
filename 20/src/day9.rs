#[aoc(day9, part1)]
fn day9_part1(input: &str) -> i64 {
	let mut input = input.trim().split("\n").map(|n| n.parse::<i64>().unwrap());
	let mut last25 = input.by_ref().take(25).collect::<std::collections::VecDeque<_>>();
	'nextInput: for v in input {
		for ai in 0 .. 25 {
			for bi in 0..25 {
				if ai != bi {
					if last25[ai] + last25[bi] == v {
						last25.pop_front();
						last25.push_back(v);
						continue 'nextInput;
					}
				}
			}
		}
		return v;
	}
	panic!();
}

#[aoc(day9, part2)]
fn day9_part2(input: &str) -> i64 {
	let bad = day9_part1(input);
	let input = input.trim().split("\n").map(|n| n.parse::<i64>().unwrap());
	let mut past = std::collections::VecDeque::new();
	let mut sum = 0;
	for v in input {
		sum += v;
		past.push_back(v);
		while sum > bad {
			sum -= past.pop_front().unwrap();
		}
		if sum == bad {
			return past.iter().min().unwrap() + past.iter().max().unwrap()
		}
	}
	panic!();
}