use std::collections::HashSet;

#[aoc(day3, part1)]
fn day3_part1(input: &str) -> usize {
	input.trim().split("\n").map(|line_str| {
		let line = line_str.as_bytes();
		let mid = line.len() / 2;
		let (a, b) = (&line[0..mid], &line[mid..]);
		let aset = a.iter().copied().collect::<HashSet<u8>>();
		let same = *b.iter().filter(|&x| aset.contains(x)).next().expect(line_str);
		let score = (if same >= b'A' && same <= b'Z' {
			same - b'A' + 27
		} else {
			same - b'a' + 1
		}) as usize;
		score
	}).sum()
}

#[test]
fn test_p1() {
	assert_eq!(day3_part1("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"), 157);
}


#[aoc(day3, part2)]
fn day3_part2(input: &str) -> usize {
	input.trim().split("\n").array_chunks::<3>().map(|line_strs| {
		let mut bit: Vec<u8> = line_strs[0].as_bytes().into();
		for line in &line_strs[1..] {
			let set = line.as_bytes().iter().copied().collect::<HashSet<u8>>();
			bit = bit.into_iter().filter(|x| set.contains(x)).collect();
		}
		let same = bit[0];
		let score = (if same >= b'A' && same <= b'Z' {
			same - b'A' + 27
		} else {
			same - b'a' + 1
		}) as usize;
		score
	}).sum()
}