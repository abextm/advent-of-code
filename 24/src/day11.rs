use std::collections::HashMap;

fn transform(cache: &mut HashMap<u64, usize>, depth: isize, val: u64) -> usize {
	if depth == 0 {
		return 1
	}

	if val == 0 {
		return transform(cache, depth - 1, 1);
	}

	let num_digits = val.ilog10() + 1;
	if (num_digits & 1) == 0 {
		let half = 10u64.pow(num_digits / 2);
		let key = ((depth as u64) << 56) | val;
		if let Some(v) = cache.get(&key) {
			*v
		} else {
			let v = transform(cache, depth - 1, val % half) +
			transform(cache, depth - 1, val / half);
			cache.insert(key, v);
			v
		}
	} else {
		transform(cache, depth - 1, val * 2024)
	}
}

#[aoc(part1=193899, part1=229682160383225)]
fn solve(input: &str, part1: bool) -> impl std::fmt::Debug {
	let mut sum = 0;
	let mut cache = HashMap::new();
	for v in input.trim().split(" ") {
		sum += transform(&mut cache, if part1 { 25 } else { 75 }, v.parse().unwrap());
		print!(".");
	}
	sum
}