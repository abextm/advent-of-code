struct Seq(i32);

impl Iterator for Seq {
	type Item = i32;

	fn next(&mut self) -> Option<Self::Item> {
		let mut secret = self.0;
		secret = (secret ^ (secret << 6)) & 0xFFFFFF;
		secret = (secret ^ (secret >> 5)) & 0xFFFFFF;
		secret = (secret ^ (secret << 11)) & 0xFFFFFF;
		self.0 = secret;
		Some(secret)
	}
}

#[aoc(part1 = 12664695565)]
fn part1(input: &str) -> impl std::fmt::Debug {
	input.trim().lines().map(|l| {
		Seq(l.parse().unwrap())
			.skip(1999)
			.next()
			.unwrap() as i64
	}).sum::<i64>()
}

#[aoc(part2 = 1444)]
fn part2(input: &str) -> u64 {
	let mut count = [0; 20 * 20 * 20 * 20];
	let mut visited = [-1i8; 20 * 20 * 20 * 20];
	input.trim().lines().enumerate().for_each(|(i, l)| {
		Seq(l.parse().unwrap()).take(2000)
			.map(|n| n % 10)
			.map_windows(|w: &[_; 5]| {
				let index = (w[1] - w[0] + 10) * (20 * 20 * 20)
					+ (w[2] - w[1] + 10) * (20 * 20)
					+ (w[3] - w[2] + 10) * 20
					+ (w[4] - w[3] + 10);

				(i as i8, index, w[4])
			})
			.for_each(|(i, index, n)| {
				let index = index as usize;
				if visited[index] != i {
					visited[index] = i;
					count[index] += n as u64
				}
			});
	});

	count.into_iter().max().unwrap()
}

#[aoc(part1 = 37327623)]
const EX: &str = "1
10
100
2024
";

#[aoc(part2 = 23)]
const EX2: &str = "1
2
3
2024
";