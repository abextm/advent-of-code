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
	((v >> 3) & 0b11) as usize
}

const fn map_name(s: &[u8]) -> usize {
	let mut name = [b'{'; 3];
	name[0] = s[0];
	if s.len() > 1 {
		name[1] = s[1];
	}
	if s.len() > 2 {
		name[2] = s[2];
	}
	name[0] |= b'a' - b'A'; // lowercase A & R
	const BASE: usize = (1 + b'{' - b'a') as usize;
	(name[0] - b'a') as usize * BASE * BASE + (name[1] - b'a') as usize * BASE + (name[2] - b'a') as usize
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

const REJECT: usize = map_name(b"R");
const ACCEPT: usize = map_name(b"A");

fn parse4(s: &str) -> usize {
	// input always has plenty of trailing data for this to be within bounds
	// only works on x86
	let mut packed = unsafe { *std::mem::transmute::<_, *const u32>(s.as_ptr()) };
	const ZEROES: u32 = 0x30303030;
	packed -= ZEROES;
	packed <<= (4 - s.len()) * 8;
	let packed10 = packed * 10;
	let v = (packed >> 24) + (((packed >> 8) & 0xFF) + (packed10 & 0xFF)) * 100 + ((packed10 >> 16) & 0xFF);
	v as usize
}

fn calculate(workflows: &[u16], input: &str, mut flow_name: usize, mut ranges: [Range; 4]) -> usize {
	let mut sum = 0;

	'workflow_loop: loop {
		if flow_name == REJECT {
			return sum;
		} else if flow_name == ACCEPT {
			return sum + ranges.iter().map(|r| r.len()).reduce(|a, b| a * b).unwrap()
		}

		let mut workflow_str = &input[workflows[flow_name] as usize..];
		loop {
			let workflow_b = workflow_str.as_bytes();
			let is_gt = match workflow_b[1] {
				b'>' => true,
				b'<' => false,
				_ => {
					// eol
					let name = memchr_take(b'}', &mut workflow_str).unwrap();
					flow_name = map_name(name.as_bytes()); // strip trailing }
					continue 'workflow_loop;
				},
			};

			let prop = map_xmas(workflow_b[0]);

			workflow_str = &workflow_str[2..];
			let rhs_str = memchr_take(b':', &mut workflow_str).unwrap();
			let target_str = memchr_take(b',', &mut workflow_str).unwrap();

			let rhs = parse4(rhs_str);

			let (match_range, rem_range) = match is_gt {
				true => (Range{min: rhs + 1, max: 4000 }, Range{min: 1, max: rhs }),
				false => (Range{min: 1, max: rhs - 1}, Range{min: rhs, max: 4000 }),
			};

			let mut match_ranges = ranges.clone();
			match_ranges[prop] = match_ranges[prop].intersect(&match_range);
			if match_ranges[prop].is_valid() {
				sum += calculate(workflows, input, map_name(target_str.as_bytes()), match_ranges);
			}

			ranges[prop] = ranges[prop].intersect(&rem_range);
			if !ranges[prop].is_valid() {
				return sum;
			}
		}
	}
}

#[derive(Copy, Clone, Debug)]
struct Range {
	min: usize,
	max: usize,
}

impl Range {
	fn intersect(self: &Range, b: &Range) -> Range {
		Range {
			min: self.min.max(b.min),
			max: self.max.min(b.max),
		}
	}

	fn is_valid(&self) -> bool {
		self.min < self.max
	}

	fn len(&self) -> usize {
		1 + self.max - self.min
	}
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
	let mut workflows = vec![u16::MAX; map_name(b"{{{")];
	for mut line in memchr_split(b'\n', input) {
		if line.len() == 0 {
			break;
		}

		let name = memchr_take(b'{', &mut line).unwrap();
		workflows[map_name(name.as_bytes())] = (line.as_ptr() as usize - input.as_ptr() as usize).try_into().unwrap();
	}
	let v = calculate(&workflows, input, map_name(b"in"), [Range{min: 1, max: 4000}; 4]);
	v
}

trait Sliceable {
	fn len(&self) -> usize;
	fn slice(&self, v: std::ops::Range<usize>) -> &Self;
}

impl Sliceable for str {
	fn len(&self) -> usize {
		str::len(self)
	}
	fn slice(&self, v: std::ops::Range<usize>) -> &Self {
		&self[v]
	}
}
impl<T> Sliceable for [T] {
	fn len(&self) -> usize {
		<[T]>::len(self)
	}
	fn slice(&self, v: std::ops::Range<usize>) -> &Self {
		&self[v]
	}
}

struct SplitByIndexIterator<'a, S, I>
where
	S: Sliceable + ?Sized,
	I: Iterator<Item=usize>
{
	start: usize,
	slice: &'a S,
	it: I,
}

impl<'a, S, I> SplitByIndexIterator<'a, S, I>
where
	S: Sliceable + ?Sized,
	I: Iterator<Item=usize>
{
	fn new(v: &'a S, it: I) -> Self {
		SplitByIndexIterator {
			start: 0,
			slice: v,
			it,
		}
	}
}

impl<'a, S, I> Iterator for SplitByIndexIterator<'a, S, I>
where
	S: Sliceable + ?Sized,
	I: Iterator<Item=usize>,
{
	type Item = &'a S;
	fn next(&mut self) -> Option<Self::Item> {
		let len = self.slice.len();
		if self.start >= len {
			return None
		}

		let end = self.it.next().unwrap_or(len);
		let v = self.slice.slice(self.start..end);
		self.start = end + 1; // skip delimiter
		Some(v)
	}
}

fn memchr_split<'a, S>(needle: u8, haystack: &'a S) -> SplitByIndexIterator<'a, S, memchr::Memchr<'a>>
where
	S: AsRef<[u8]> + Sliceable + ?Sized,
{
	SplitByIndexIterator::new(haystack, memchr::memchr_iter(needle, haystack.as_ref()))
}

fn memchr_take<'a>(needle: u8, str: &mut &'a str) -> Option<&'a str> {
	// this is faster than memchr for small strings since there isn't any dynamic dispatch
	match str.as_bytes().iter().position(|&x| x == needle) {
		None => None,
		Some(index) => {
			unsafe {
				let m = str.get_unchecked(..index);
				*str = str.get_unchecked(index + 1..);
				Some(m)
			}
		}
	}
}
