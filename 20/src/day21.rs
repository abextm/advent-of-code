#[aoc(day21, part1)]
fn day21_part1(input: &str) -> usize {
	let match_contains = regex::Regex::new("([^(]+) \\(contains ([^)]+)\\)").unwrap();
	let mut possible_alergens = std::collections::HashMap::new();
	let mut all_ingred = std::collections::HashSet::new();
	let mut all_ingred_dup = Vec::new();
	for line in input.trim().split("\n") {
		let c = match_contains.captures(line).expect(line);
		let ingredients = c[1].split(" ").map(|x|x.into()).collect::<std::collections::HashSet<String>>();
		let alergens = c[2].split(", ").map(|x|x.into()).collect::<std::collections::HashSet<String>>();
		all_ingred_dup.extend(ingredients.iter().cloned());
		all_ingred.extend(ingredients.iter().cloned());
		for al in alergens.iter() {
			match possible_alergens.entry(al.clone()) {
				std::collections::hash_map::Entry::Vacant(v) => {
					v.insert(ingredients.clone());
				},
				std::collections::hash_map::Entry::Occupied(mut v) => {
					let val = v.get().iter().cloned().filter(|i| ingredients.contains(i)).collect();
					v.insert(val);
				},
			}
		}
	}

	println!("{:?}", possible_alergens);

	let mut not_alergens = all_ingred.clone();
	for als in possible_alergens.values() {
		for al in als.iter() {
			not_alergens.remove(al);
		}
	}

	println!("{:?}", not_alergens);

	all_ingred_dup.iter().filter(|x| not_alergens.contains(&x[..])).count()
}

#[aoc(day21, part2)]
fn day21_part2(input: &str) -> String {
	let match_contains = regex::Regex::new("([^(]+) \\(contains ([^)]+)\\)").unwrap();
	let mut possible_alergens = std::collections::HashMap::new();
	let mut all_ingred = std::collections::HashSet::new();
	let mut all_ingred_dup = Vec::new();
	for line in input.trim().split("\n") {
		let c = match_contains.captures(line).expect(line);
		let ingredients = c[1].split(" ").map(|x|x.into()).collect::<std::collections::HashSet<String>>();
		let alergens = c[2].split(", ").map(|x|x.into()).collect::<std::collections::HashSet<String>>();
		all_ingred_dup.extend(ingredients.iter().cloned());
		all_ingred.extend(ingredients.iter().cloned());
		for al in alergens.iter() {
			match possible_alergens.entry(al.clone()) {
				std::collections::hash_map::Entry::Vacant(v) => {
					v.insert(ingredients.clone());
				},
				std::collections::hash_map::Entry::Occupied(mut v) => {
					let val = v.get().iter().cloned().filter(|i| ingredients.contains(i)).collect();
					v.insert(val);
				},
			}
		}
	}

	println!("{:?}", possible_alergens);
	let mut known_alergens = std::collections::HashSet::new();
	let mut possible_alergens = possible_alergens.iter().collect::<Vec<_>>();
	possible_alergens.sort_by_key(|(_, x)| x.len());
	let mut out = Vec::new();
	for (readable, possible) in possible_alergens.iter() {
		println!("{:?} {:?}", readable, possible);
		let cannon = possible.iter().filter(|x| !known_alergens.contains(&x[..])).next().unwrap();
		known_alergens.insert(cannon.clone());
		out.push((readable.clone(), cannon));
	}

	out.sort_by_key(|(k, _)| k.clone());

	println!("{:?}", out);

	out.into_iter().map(|(_, v)| v.clone()).collect::<Vec<_>>().join(",")
}