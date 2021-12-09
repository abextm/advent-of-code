use std::cmp::min;

#[aoc(day7, part1)]
fn day7_part1(input: &str) -> isize {
	let mut input: Vec<_> = input.split(",").map(|x|x.parse::<isize>().unwrap()).collect();
	input.sort();
	let mut avg = input[input.len() / 2];
	let a  = input.iter().map(|&x| ((x as isize - avg as isize) as isize).abs()).sum();
	avg -= 1;
	let b  = input.iter().map(|&x| ((x as isize - avg as isize) as isize).abs()).sum();
	std::cmp::min(a, b)
}

#[test]
fn test_p1() {
	assert_eq!(37, day7_part1("16,1,2,0,4,2,7,1,2,14"))
}
#[aoc(day7, part2)]
fn day7_part2(input: &str) -> isize {
	let mut input: Vec<_> = input.split(",").map(|x|x.parse::<isize>().unwrap()).collect();
	let mut out = isize::max_value();
	for avg in (*input.iter().min().unwrap())..=(*input.iter().max().unwrap()) {
		let v = input.iter().map(|&x| {
			let dist = (x - avg).abs();
			(1..=dist).sum::<isize>()
		}).sum::<isize>();
		out = min(v, out);
	}
	out
}

#[test]
fn test_p2() {
	assert_eq!(168, day7_part2("16,1,2,0,4,2,7,1,2,14"))
}