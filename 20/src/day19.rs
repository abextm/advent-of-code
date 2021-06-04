use regex::Regex;

#[aoc(day19, part1)]
fn day19_part1(input: &str) -> usize {
	let mut input = input.trim().split("\n\n");

	let rules = parse_rules(input.next().unwrap());

	let regex = to_regex(&rules, rules.get(&0).unwrap());
	let regex = Regex::new(&format!("^{}$", regex)).unwrap();

	input
		.next()
		.unwrap()
		.split("\n")
		.filter(|line| regex.is_match(line))
		.count()
}

fn parse_rules(input: &str) -> std::collections::HashMap<i64, Rule> {
	let match_key = Regex::new("^([0-9]+):+ (.*)$").unwrap();
	let match_or = Regex::new("^([0-9 ]+) \\| ([0-9 ]+)$").unwrap();
	let match_quotes = Regex::new("^\"([a-z])\"$").unwrap();
	let match_seq = Regex::new("^([0-9 ]+)$").unwrap();
	let mut rules = std::collections::HashMap::new();
	for line in input.split("\n") {
		let c = match_key.captures(line).unwrap();
		let id = c[1].parse::<i64>().unwrap();
		let line = &c[2];

		let rule = if let Some(c) = match_or.captures(line) {
			Rule::Or(
				c[1].split(" ").map(|x| x.parse().unwrap()).collect(),
				c[2].split(" ").map(|x| x.parse().unwrap()).collect(),
			)
		} else if let Some(c) = match_seq.captures(line) {
			Rule::Seq(c[1].split(" ").map(|x| x.parse().unwrap()).collect())
		} else if let Some(c) = match_quotes.captures(line) {
			Rule::Char(c[1].chars().next().unwrap())
		} else {
			panic!("{}", line);
		};

		rules.insert(id, rule);
	}
	rules
}

#[derive(Clone)]
enum Rule {
	Char(char),
	Seq(Vec<i64>),
	Or(Vec<i64>, Vec<i64>),
}

fn to_regex(rules: &std::collections::HashMap<i64, Rule>, rule: &Rule) -> String {
	match rule {
		Rule::Char(c) => c.to_string(),
		Rule::Seq(w) => w
			.iter()
			.map(|a| to_regex(rules, rules.get(a).unwrap()))
			.collect::<Vec<_>>()
			.join(""),
		Rule::Or(a, b) => {
			format!(
				"({}|{})",
				a.iter()
					.map(|a| to_regex(rules, rules.get(a).unwrap()))
					.collect::<Vec<_>>()
					.join(""),
				b.iter()
					.map(|a| to_regex(rules, rules.get(a).unwrap()))
					.collect::<Vec<_>>()
					.join(""),
			)
		}
	}
}

#[aoc(day19, part2)]
fn day19_part2(input: &str) -> usize {
	let mut input = input.trim().split("\n\n");

	let rules_str = format!(
		"{}\n{}",
		input.next().unwrap(),
		"8: 42 | 42 8
11: 42 31 | 42 11 31"
	);
	let rules = parse_rules(&rules_str);

	input
		.next()
		.unwrap()
		.split("\n")
		.filter(|line| match test_rules(&rules, rules.get(&0).unwrap(), line) {
			Some(v) if v.len() > 0 => v.iter().any(|x| x.len() == 0),
			_ => false,	
		})
		.count()
}

fn test_rules<'a>(
	rules: &std::collections::HashMap<i64, Rule>,
	rule: &Rule,
	input: &'a str,
) -> Option<Vec<&'a str>> {
	match rule {
		&Rule::Char(c) => {
			if Some(c) == input.chars().next() {
				Some(vec![&input[1..]])
			} else {
				None
			}
		}
		Rule::Seq(seq) => test_seq(rules, &seq[..], input),
		Rule::Or(a, b) => {
			let mut out = Vec::new();
			if let Some(mut v) = test_seq(rules, &a[..], input) {
				out.append(&mut v);
			}
			if let Some(mut v) = test_seq(rules, &b[..], input) {
				out.append(&mut v);
			}
			if out.len() > 0 {
				Some(out)
			} else {
				None
			}
		}
	}
}

fn test_seq<'a>(
	rules: &std::collections::HashMap<i64, Rule>,
	rule: &[i64],
	input: &'a str,
) -> Option<Vec<&'a str>> {
	let remainders = &rule[1..];
	let rule = rule[0];
	if let Some(s) = test_rules(rules, rules.get(&rule).unwrap(), input) {
		if remainders.len() > 0 {
			let out = s
				.iter()
				.filter_map(|s| test_seq(rules, remainders, s))
				.flat_map(|x| x.into_iter())
				.collect::<Vec<_>>();
			if out.len() > 0 {
				Some(out)
			} else {
				None
			}
		} else {
			Some(s)
		}
	} else {
		None
	}
}
