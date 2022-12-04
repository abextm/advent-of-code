
#[aoc(day4, part1)]
fn day4_part1(input: &str) -> usize {
	input.trim().lines().filter(|l| {
		let mut it = l.split(",").map(|x| {
			let mut it = x.split("-").map(|v| v.parse::<usize>().unwrap());
			let lhs = it.next().unwrap();
			let rhs = it.next().unwrap();
			return (lhs, rhs);
		});
		let lhs = it.next().unwrap();
		let rhs = it.next().unwrap();

		return lhs.0 <= rhs.0 && lhs.1 >= rhs.1 || rhs.0 <= lhs.0 && rhs.1 >= lhs.1;
	}).count()
}

#[aoc(day4, part2)]
fn day4_part2(input: &str) -> usize {
	input.trim().lines().filter(|l| {
		let mut it = l.split(",").map(|x| {
			let mut it = x.split("-").map(|v| v.parse::<usize>().unwrap());
			let lhs = it.next().unwrap();
			let rhs = it.next().unwrap();
			return (lhs, rhs);
		});
		let mut lhs = it.next().unwrap();
		let mut rhs = it.next().unwrap();

		if lhs.0 > rhs.0 {
			std::mem::swap(&mut lhs, &mut rhs);
		}

		lhs.1 >= rhs.0
		
	}).count()
}