#[aoc(day16, part1)]
fn day16_part1(input: &str) -> i64 {
	let rule_matcher =
		regex::Regex::new("^([^:]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
	let parts = input.trim().split("\n\n").collect::<Vec<_>>();
	let rules = parts[0]
		.trim()
		.split("\n")
		.map(|line| {
			let line = rule_matcher.captures(line).unwrap();
			let name = &line[1];
			let la = line[2].parse::<i64>().unwrap();
			let ha = line[3].parse::<i64>().unwrap();
			let lb = line[4].parse::<i64>().unwrap();
			let hb = line[5].parse::<i64>().unwrap();
			(name.into(), (la, ha, lb, hb))
		})
		.collect::<Vec<(String, _)>>();

	parts[2]
		.split("\n")
		.skip(1)
		.map(parse_commas)
		.flat_map(|l|
			l.into_iter().filter(|&v| {
				!rules.iter().any(|(_, rule)| (v >= rule.0 && v <= rule.1) || (v >= rule.2 && v <= rule.3))
			})
		)
		.sum()
}

#[test]
fn p1() {
	assert_eq!(day16_part1("class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"), 71);
}

fn parse_commas(s: &str) -> Vec<i64> {
	s.split(",").map(|x| x.parse().unwrap()).collect()
}

#[aoc(day16, part2)]
fn day16_part2(input: &str) -> i64 {
	let rule_matcher =
		regex::Regex::new("^([^:]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
	let parts = input.trim().split("\n\n").collect::<Vec<_>>();
	let mut rules = parts[0]
		.trim()
		.split("\n")
		.map(|line| {
			let line = rule_matcher.captures(line).unwrap();
			let name = &line[1];
			let la = line[2].parse::<i64>().unwrap();
			let ha = line[3].parse::<i64>().unwrap();
			let lb = line[4].parse::<i64>().unwrap();
			let hb = line[5].parse::<i64>().unwrap();
			(name.into(), (la, ha, lb, hb))
		})
		.collect::<Vec<(String, _)>>();

	let our_ticket = parse_commas(parts[1].split("\n").skip(1).next().unwrap());

	let tickets = parts[2]
		.split("\n")
		.skip(1)
		.map(parse_commas)
		.filter(|tick|
			tick.iter().all(|&v| {
				rules.iter().any(|(_, rule)| (v >= rule.0 && v <= rule.1) || (v >= rule.2 && v <= rule.3))
			})
		)
		.collect::<Vec<_>>();

	let num_fields = tickets[0].len();
	let mut rule_validity = rules.iter().map(|(rule_name, rule)| {
		(rule_name, (0..num_fields).filter(|&field| {
			tickets.iter().all(|tick| {
				let v = tick[field];
				(v >= rule.0 && v <= rule.1) || (v >= rule.2 && v <= rule.3)
			})
		}).collect::<Vec<usize>>())
	}).collect::<Vec<_>>();
	rule_validity.sort_by_key(|(_, r)| r.len());

	// rule_name -> column
	let mut rules = std::collections::HashMap::<String, usize>::new();
	let mut used_columns = std::collections::HashSet::new();
	for (rule_name, columns) in rule_validity {
		let col = *columns.iter().filter(|&c| !used_columns.contains(c) ).next().unwrap();
		used_columns.insert(col);
		rules.insert(rule_name.clone(), col);
	}

	println!("{:?}", rules);

	let mut v = 1;
	for (_, &col) in rules.iter().filter(|(name, _)| name.starts_with("departure")) {
		v *= our_ticket[col];
	}

	v
}

#[test]
fn p2() {
	assert_eq!(day16_part2("departure: 0-1 or 4-19
departure also: 0-5 or 8-19
foo: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"), 12 * 11)
}
