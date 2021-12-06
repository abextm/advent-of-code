#[aoc(day6, part1)]
fn day6_part1(input: &str) -> usize {
	solve(input, 80)
}
#[aoc(day6, part2)]
fn day6_part2(input: &str) -> usize {
	solve(input, 256)
}
fn solve(input: &str, days: usize) -> usize {
	let mut state = [0; 9];
	for fish in input.trim().split(",") {
		state[fish.parse::<usize>().unwrap()] += 1;
	}
	for day in 0..days {
		state[(day + 7) % state.len()] += state[day % state.len()];
	}
	state.iter().sum()
}

#[test]
fn test_part1() {
	assert_eq!(day6_part1("3,4,3,1,2"), 5934);
}