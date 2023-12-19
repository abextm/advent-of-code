use std::collections::HashMap;

struct Rule<'a> {
	prop: usize,
	op: u8,
	rhs: usize,
	dst: &'a str,
}

struct Workflow<'a> {
	rules: Vec<Rule<'a>>,
	fallback: &'a str,
}

fn map_xmas(v: u8) -> usize {
	match v {
		b'x' => 0,
		b'm' => 1,
		b'a' => 2,
		b's' => 3,
		_ => panic!(),
	}
}

fn parse_workflows<'a, I: Iterator<Item=&'a str>>(lines: &mut I) -> HashMap<&'a str, Workflow<'a>> {
	let rule_pattern = regex::Regex::new(r#"^([xmas])([<>])([0-9]+):([a-zA-Z]+)$"#).unwrap();
	let mut workflows = HashMap::new();
	for line in lines {
		if line.len() == 0 {
			break;
		}
		let (name, rem) = line.split_once("{").unwrap();
		let rem = &rem[..rem.len() - 1];
		let (rem, fallback) = rem.rsplit_once(",").unwrap();
		let rules = rem.split(",").map(|rule_str| {
			let cap = rule_pattern.captures(rule_str).unwrap();
			Rule {
				prop: map_xmas(cap[1].as_bytes()[0]),
				op: cap[2].as_bytes()[0],
				rhs: cap[3].parse().unwrap(),
				dst: &rule_str[cap.get(4).unwrap().range()],
			}
		}).collect();
		workflows.insert(name, Workflow {
			rules,
			fallback,
		});
	}
	workflows
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
	let mut input = input.lines();
	let workflows = parse_workflows(&mut input);
	input.map(|part_str| {
		let mut values = [0usize; 4];
		for val in part_str[1..part_str.len() - 1].split(",") {
			let (key, val) = val.split_once("=").unwrap();
			values[map_xmas(key.as_bytes()[0])] = val.parse().unwrap();
		}

		let mut flow_name = "in";
		loop {
			let flow = workflows.get(flow_name).expect(flow_name);
			flow_name = flow.rules.iter().filter_map(|rule| {
				let lhs = values[rule.prop];
				let matches = match rule.op {
					b'>' => lhs > rule.rhs,
					b'<' => lhs < rule.rhs,
					_ => panic!(),
				};
				if matches {
					Some(rule.dst)
				} else {
					None
				}
			}).next().unwrap_or(flow.fallback);
			if flow_name == "R" {
				return 0;
			} else if flow_name == "A" {
				return values.iter().sum();
			}
		}
	}).sum()
}

fn calculate<'a>(workflows: &HashMap<&'a str, Workflow<'a>>, flow_name: &'a str, mut ranges: [Range; 4]) -> usize {
	if flow_name == "R" {
		return 0;
	} else if flow_name == "A" {
		return ranges.iter().map(|r| r.max + 1 - r.min).reduce(|a, b| a * b).unwrap()
	}

	let workflow = workflows.get(flow_name).unwrap();

	let mut sum = 0;
	for rule in workflow.rules.iter() {
		let (match_range, rem_range) = match rule.op {
			b'>' => (Range{min: rule.rhs + 1, max: 4000 }, Range{min: 1, max: rule.rhs }),
			b'<' => (Range{min: 1, max: rule.rhs - 1}, Range{min: rule.rhs, max: 4000 }),
			_ => panic!(),
		};

		let range = ranges[rule.prop];
		if let Some(v) = intersect(&match_range, &range) {
			let mut match_ranges = ranges.clone();
			match_ranges[rule.prop] = v;
			sum += calculate(workflows, rule.dst, match_ranges);
		}
		if let Some(v) = intersect(&rem_range, &range) {
			ranges[rule.prop] = v;
		} else {
			return sum;
		}
	}

	sum + calculate(workflows, workflow.fallback, ranges)
}

#[derive(Copy, Clone, Debug)]
struct Range {
	min: usize,
	max: usize,
}

fn intersect(a: &Range, b: &Range) -> Option<Range> {
	let r = Range {
		min: a.min.max(b.min),
		max: a.max.min(b.max),
	};
	if r.min <= r.max {
		Some(r)
	} else {
		None
	}
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
	let mut input = input.lines();
	let workflows = parse_workflows(&mut input);
	calculate(&workflows, "in", [Range{min: 1, max: 4000}; 4])
}