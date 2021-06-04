#[aoc(day7, part1)]
fn day7_part1(input: &str) -> usize {
	let bag_re = regex::Regex::new("(.*) bags contain (.*)\\.").unwrap();
	let count_re = regex::Regex::new("([0-9]+) (.*) bags?").unwrap();

	let mut contains = std::collections::HashMap::<String, Vec<String>>::new();
	for line in input.trim().split("\n") {
		let cap = bag_re.captures(line).expect(line);
		let key = &cap[1];
		let baglist = &cap[2];
		if baglist.ends_with("no other bags") {
			continue
		}
		for num_bag in baglist.split(", ") {
			let cap = count_re.captures(num_bag).expect(num_bag);
			let name = &cap[2];
			contains.entry(name.into())
				.or_default()
				.push(key.into());
		}
	}

	let mut out = std::collections::HashSet::new();
	get_parents(&mut out, &contains,  "shiny gold");
	out.len()
}

fn get_parents(out: &mut std::collections::HashSet<String>, contains: &std::collections::HashMap<String, Vec<String>>, want: &str) {
	match contains.get(want) {
		Some(v) => {
			for p in v {
				if out.insert(p.clone()) {
					get_parents(out, contains, p);
				}
			}
		}
		None => (),
	};
}

#[aoc(day7, part2)]
fn day7_part2(input: &str) -> usize {
	let bag_re = regex::Regex::new("(.*) bags contain (.*)\\.").unwrap();
	let count_re = regex::Regex::new("([0-9]+) (.*) bags?").unwrap();

	let mut contents = std::collections::HashMap::<String, Vec<(usize, String)>>::new();
	for line in input.trim().split("\n") {
		let cap = bag_re.captures(line).expect(line);
		let key = &cap[1];
		let baglist = &cap[2];
		let vec = contents.entry(key.into())
			.or_default();
		if baglist.ends_with("no other bags") {
			continue
		}
		for num_bag in baglist.split(", ") {
			let cap = count_re.captures(num_bag).expect(num_bag);
			let amnt = &cap[1];
			let name = &cap[2];
			vec.push((amnt.parse().unwrap(), name.into()));
		}
	}

	get_children(&contents, "shiny gold")
}

fn get_children(contents: &std::collections::HashMap<String, Vec<(usize, String)>>, want: &str) -> usize {
	match contents.get(want) {
		Some(v) => {
			let mut count = 0;
			for p in v {
				count += p.0 * (1 + get_children(contents, &p.1));
			}
			count
		}
		None => panic!("{}", want),
	}
}