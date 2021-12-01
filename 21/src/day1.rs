#[aoc(day1, part1)]
fn day1_part1(input: &str) -> usize {
	let v = input.trim().split("\n")
		.map(|x| x.parse::<i64>().unwrap())
		.collect::<Vec<_>>();
	
	v.iter().zip(v.iter().skip(1)).filter(|(p, n)| n > p).count()
}
#[aoc(day1, part2)]
fn day1_part2(input: &str) -> usize {
	let v = input.trim().split("\n")
		.map(|x| x.parse::<i64>().unwrap())
		.collect::<Vec<_>>();
	
	let v2 = v.iter()
		.zip(v.iter().skip(1))
		.zip(v.iter().skip(2))
		.map(|((a, b), c)| a + b + c)
		.collect::<Vec<_>>();

	v2.iter().zip(v2.iter().skip(1)).filter(|(p, n)| n > p).count()
}
