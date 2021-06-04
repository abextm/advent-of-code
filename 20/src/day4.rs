use std::collections::HashMap;

#[aoc(day4, part1)]
fn day4_part1(input: &str) -> usize {
	input
		.split("\n\n")
		.filter(|raw| {
			let mut map = HashMap::new();
			for kvraw in raw.split(&[' ', '\n'][..]) {
				let kv = kvraw.splitn(2, ':').collect::<Vec<_>>();
				map.insert(kv[0], kv[1]);
			}
			["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
				.iter()
				.all(|c| map.contains_key(c))
		})
		.count()
}

type Tester = fn(&regex::Captures) -> bool;
#[aoc(day4, part2)]
fn day4_part2(input: &str) -> usize {
	let tests: &[(_, _, Tester)] = &[
		("byr", "[0-9]+", |c| {
			(1920..=2002).contains(&c[0].parse::<i32>().unwrap())
		}),
		("iyr", "[0-9]+", |c| {
			(2010..=2020).contains(&c[0].parse::<i32>().unwrap())
		}),
		("eyr", "[0-9]+", |c| {
			(2020..=2030).contains(&c[0].parse::<i32>().unwrap())
		}),
		("hgt", "([0-9]+)(cm|in)", |c| {
			if *"in" == c[2] { 59..=76 } else { 150..=193 }.contains(&c[1].parse::<i32>().unwrap())
		}),
		("hcl", "#([0-9a-f]{6})", |_c| true),
		("ecl", "amb|blu|brn|gry|grn|hzl|oth", |_c| true),
		("pid", "[0-9]{9}", |_c| true),
	];
	let tests = tests
		.iter()
		.map(|&(key, pat, test)| (key, regex::Regex::new(&format!("^{}$", pat)).expect(key), test))
		.collect::<Vec<_>>();

	input
		.split("\n\n")
		.filter(|raw| {
			let mut map = HashMap::new();
			for kvraw in raw.split(&[' ', '\n'][..]) {
				let kv = kvraw.splitn(2, ':').collect::<Vec<_>>();
				map.insert(kv[0], kv[1]);
			}
			for (key, re, test) in tests.iter() {
				let v = match map.get(key) {
					None => return false,
					Some(v) => v,
				};
				let v = match re.captures(v) {
					None => return false,
					Some(v) => v,
				};
				if !test(&v) {
					return false;
				}
			}
			println!("{:?}", map.get("hgt"));
			true
		})
		.count()
}
