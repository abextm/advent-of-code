#[aoc(day2, part1)]
fn day2_part1(input: &str) -> usize {
	input.trim().split("\n").map(|line| {
		let opponent = line.bytes().nth(0).unwrap() - b'A';
		let me = line.bytes().nth(2).unwrap() - b'X';

		let score = me + 1;
		let win = 3 * ((me + 4 - opponent) % 3);

		return (win + score) as usize;
	}).sum()
}

#[aoc(day2, part2)]
fn day2_part2(input: &str) -> usize {
	input.trim().split("\n").map(|line| {
		let opponent = line.bytes().nth(0).unwrap() - b'A';
		let outcome = line.bytes().nth(2).unwrap() - b'X';

		let win = (outcome * 3) as usize;
		let me = (outcome + 2 + opponent) % 3;

		return win + (me + 1) as usize
	}).sum()
}