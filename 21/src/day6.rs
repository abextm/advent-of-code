#[aoc(day6, part1)]
fn day6_part1(input: &str) -> usize {
	solve(input, 80)
}
#[aoc(day6, part2)]
fn day6_part2(input: &str) -> usize {
	solve(input, 256)
}
fn solve(input: &str, days: usize) -> usize {
	let mut state: Vec<usize> = vec![0; 9];
	let mut next_state = state.clone();
	for fish in input.trim().split(",").map(|x| x.parse::<usize>().unwrap()) {
		state[fish] += 1;
	}
	for _day in 0..days {
		next_state.fill(0);
		for lifespan in 1..=8 {
			next_state[lifespan - 1] += state[lifespan];
		}
		next_state[6] += state[0];
		next_state[8] += state[0];
		std::mem::swap(&mut state, &mut next_state);
	}
	state.iter().sum()
}

#[test]
fn test_part1() {
	assert_eq!(day6_part1("3,4,3,1,2"), 5934);
}