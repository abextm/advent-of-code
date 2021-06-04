#[aoc(day2, part1)]
fn day2_part1(input: &str) -> usize {
	let re = regex::Regex::new("([0-9]+)-([0-9]+) ([a-z]): (.*)").unwrap();
	input
		.trim()
		.split("\n")
		.filter(|l| {
			let cap = re.captures(l).unwrap();
			let min: usize = cap[1].parse().unwrap();
			let max: usize = cap[2].parse().unwrap();
			let c = &cap[3];
			let mut pw = &cap[4];
			let mut count = 0;
			loop {
				match pw.find(c) {
					Some(idx) => pw = &pw[idx + 1..],
					None => break,
				}
				count += 1;
			}
			count >= min && count <= max
		})
		.count()
}

#[aoc(day2, part2)]
fn day2_part2(input: &str) -> usize {
	let re = regex::Regex::new("([0-9]+)-([0-9]+) ([a-z]): (.*)").unwrap();
	input
		.trim()
		.split("\n")
		.filter(|l| {
			let cap = re.captures(l).unwrap();
			let a: usize = cap[1].parse::<usize>().unwrap() - 1;
			let b: usize = cap[2].parse::<usize>().unwrap() - 1;
			let c = cap[3].as_bytes()[0];
			let pw = &cap[4].as_bytes();
			let aok = pw.len() > a && pw[a] == c;
			let bok = pw.len() > b && pw[b] == c;
			(aok || bok) && !(aok && bok)
		})
		.count()
}
