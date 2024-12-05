use std::collections::{HashMap, HashSet};

#[aoc(part1 = 5091)]
fn solve(input: &str) -> impl std::fmt::Debug {
	let [rules, bit] = input.trim().splitn(2, "\n\n").next_chunk().unwrap();

	// page -> cannot be after
	let mut rule_map = HashMap::<u8, HashSet<u8>>::new();
	for line in rules.lines() {
		let [a, b] = line.splitn(2, "|").map(|p| p.parse().unwrap()).next_chunk().unwrap();
		// a before b
		rule_map.entry(b).or_default().insert(a);
	}

	bit.lines().map(|l| {
		let pages = l.split(",").map(|n| n.parse::<u8>().unwrap()).collect::<Vec<_>>();

		for (i, pg) in pages.iter().enumerate() {
			if let Some(no_after) = rule_map.get(&pg) {
				for pg2 in pages[i + 1..].iter() {
					if no_after.contains(&pg2) {
						return 0;
					}
				}
			}
		}

		pages[pages.len() / 2] as usize
	}).sum::<usize>()
}

#[aoc(part2)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let [rules, bit] = input.trim().splitn(2, "\n\n").next_chunk().unwrap();

	// page -> cannot be after
	let mut rule_map = HashMap::<u8, HashSet<u8>>::new();
	for line in rules.lines() {
		let [a, b] = line.splitn(2, "|").map(|p| p.parse().unwrap()).next_chunk().unwrap();
		// a before b
		rule_map.entry(b).or_default().insert(a);
	}

	bit.lines().map(|l| {
		let mut pages = l.split(",").map(|n| n.parse::<u8>().unwrap()).collect::<Vec<_>>();

		let mut broken = false;
		for i in 0..pages.len() {
			'again: loop {
				let pg = pages[i];
				if let Some(no_after) = rule_map.get(&pg) {
					for i2 in (i + 1)..pages.len() {
						let p2 = pages[i2];
						if no_after.contains(&p2) {
							pages.copy_within(i..i2, i + 1);
							pages[i] = p2;
							broken = true;
							continue 'again;
						}
					}
				}
				break
			}
		}

		if broken {
			pages[pages.len() / 2] as usize
		} else {
			0
		}
	}).sum::<usize>()
}


#[aoc(part1 = 143, part2 = 123)]
const EX: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";