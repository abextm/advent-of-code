use std::collections::hash_map;
fn hash(str: &str) -> u8 {
	str.as_bytes().iter().fold(0, |acc, v| v.wrapping_add(acc).wrapping_mul(17))
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
	input.trim().split(",")
		.map(|str| hash(str) as usize)
		.sum::<usize>()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
	let pat = regex::Regex::new("(.*)(-|=([0-9]))").unwrap();

	let mut map = hash_map::HashMap::<&str, (u8, usize)>::new();
	let mut it = input.trim().split(",")
		.map(|str| {
			let cap = pat.captures(str).unwrap();
			let label = &str[cap.get(1).unwrap().range()];
			let focal_len = cap.get(3).map(|s| s.as_str().parse::<u8>().unwrap());
			(label, focal_len)
		});

	let mut order = Vec::<Option<&str>>::new();
	loop {
		match it.next() {
			Some((label, Some(len))) => {
				match map.entry(label) {
					hash_map::Entry::Occupied(mut v) => v.get_mut().0 = len,
					hash_map::Entry::Vacant(v) => {
						v.insert((len, order.len()));
						order.push(Some(label));
					},
				}
			},
			Some((label, None)) => {
				if let Some(ent) = map.remove(&label) {
					order[ent.1] = None;
				}
			},
			None => break,
		}	
	}

	let mut lens = vec![0usize; 256];
	let mut sum = 0;
	for label in order {
		if let Some(label) = label {
			if let Some(&(focal_length, _)) = map.get(label) {
				let hash = hash(label) as usize;
				let index = lens[hash] + 1;
				lens[hash] = index;
				sum += focal_length as usize * (hash + 1) * index;
			}
		}
	}
	sum
}