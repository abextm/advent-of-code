#[aoc(day6, part1)]
fn day6_part1(input: &str) -> usize {
	input
		.trim()
		.split("\n\n")
		.map(|group| {
			group
				.as_bytes()
				.iter()
				.cloned()
				.filter(|&c| c != '\n' as u8)
				.collect::<std::collections::HashSet<_>>()
				.len()
		})
		.sum()
}
#[aoc(day6, part2)]
fn day6_part2(input: &str) -> usize {
	input
		.trim()
		.split("\n\n")
		.map(|group| {
			('a' as u8..='z' as u8)
				.filter(|&t| {
					group
						.split("\n")
						.all(|p| p.as_bytes().iter().find(|&&c| c == t) != None)
				})
				.count()
		})
		.sum()
}
