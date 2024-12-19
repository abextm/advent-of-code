use regex::Regex;

#[aoc(part1 = 238)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let [patterns, designs] = input.trim().split("\n\n").next_chunk().unwrap();

	let re = Regex::new(&format!("^({})*$", patterns.replace(", ", "|"))).unwrap();

	designs.lines().filter(|v| re.is_match(v)).count()
}

fn match_recursive(v: &str, pats: &[&str], cache: &mut Vec<Option<usize>>) -> usize {
	if let Some(v) = cache[v.len()] {
		return v;
	}
	let mut out = 0;
	for pat in pats.iter() {
		if let Some(iv) = v.strip_prefix(pat) {
			if iv.len() == 0 {
				out += 1;
			} else {
				out += match_recursive(iv, pats, cache);
			}
		}
	}
	cache[v.len()] = Some(out);
	out
}

#[aoc(part2 = 635018909726691)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let [patterns, designs] = input.trim().split("\n\n").next_chunk().unwrap();

	let mut pats: Vec<&str> = patterns.split(", ").collect();
	pats.sort();

	pats.iter().for_each(|p| println!("{}", p));

	designs
		.lines()
		.map(|v| match_recursive(v, &pats, &mut vec![None; v.len() + 1]))
		.sum::<usize>()
}
