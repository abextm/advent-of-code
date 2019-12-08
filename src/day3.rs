use std::collections::{HashMap};

#[aoc(day3, part1)]
fn day3_part1(input: &str) -> u32 {
	day3(input, false)
}

#[aoc(day3, part2)]
fn day3_part2(input: &str) -> u32 {
	day3(input, true)
}

fn day3(input: &str, part2: bool) -> u32 {
	let mut map = HashMap::<(i32, i32), [u32; 2]>::new();

	for (lineno, linestr) in input
		.trim()
		.split("\n")
		.enumerate() {
			let mut loc = (0, 0);
			let mut dist = 0;
			for s in linestr.trim().split(",") {
				let num: i32 = s[1..].parse().unwrap();
				let dir = match &s[..1] {
					"R" => (1, 0),
					"L" => (-1, 0),
					"D" => (0, 1),
					"U" => (0, -1),
					_ => panic!("Invalid direction {}", s),
				};
				for _ in 0..num {
					dist += 1;
					loc = (loc.0 + dir.0, loc.1 + dir.1);
					let locptr = &mut map.entry(loc).or_insert([0, 0])[lineno];
					if *locptr == 0 {
						*locptr = dist;
					}
				}
			}
		}
	
	let mut v = map.iter()
		.filter(|t| t.1[0] != 0 && t.1[1] != 0)
		.map(|t| if !part2 {
			((t.0).0.abs() + (t.0).1.abs()) as u32
		} else {
			t.1[0] + t.1[1]
		})
		.collect::<Vec<_>>();
	v.sort();

	v[0]
}