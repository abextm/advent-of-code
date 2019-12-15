use crate::taken::TakeN;
use std::collections::HashMap;

#[aoc(day14, part1)]
fn day14_part1(input: &str) -> isize {
	let mut keys = decode_recipes(input);
	get(&mut keys, 1, "FUEL")
}

#[aoc(day14, part2)]
fn day14_part2(input: &str) -> isize {
	let mut keys = decode_recipes(input);
	get_max(&mut keys, TRIL, "FUEL")
}

fn decode_recipes<'a>(input: &'a str) -> HashMap<&'a str, Recipe<'a>> {
	input
		.split("\n")
		.map(|l| {
			let [input, output]: [_; 2] = l.split("=>").take_n().unwrap();
			let [omul, oname]: [_; 2] = output.trim().split(" ").take_n().unwrap();
			(
				oname.trim(),
				Recipe {
					extra: 0,
					output: omul.parse::<isize>().unwrap(),
					deps: input
						.split(",")
						.map(|s| {
							let [v, name]: [_; 2] = s.trim().split(" ").map(|s| s.trim()).take_n().unwrap();
							(v.parse::<isize>().expect(v), name)
						})
						.collect(),
				},
			)
		})
		.collect()
}

struct Recipe<'a> {
	deps: Vec<(isize, &'a str)>,
	output: isize,
	extra: isize,
}

fn get(
	map: &mut HashMap<&str, Recipe<'_>>,
	mut qty: isize,
	name: &str,
) -> isize {
	if name == "ORE" {
		qty
	} else {
		let (deps, minqty) = {
			let rec = map.get_mut(name).expect(name);
			qty -= rec.extra;
			let minqty = (qty + rec.output - 1) / rec.output;
			rec.extra = minqty * rec.output - qty;
			(rec.deps.clone(), minqty)
		};

		deps.iter()
			.map(|(mul, name)| get(map, minqty * mul, name))
			.sum()
	}
}

fn reset(map: &mut HashMap<&str, Recipe<'_>>) {
	for v in map.values_mut() {
		v.extra = 0;
	}
}

fn get_max(map: &mut HashMap<&str, Recipe<'_>>, max_ore: isize, name: &str) -> isize {
	let ore_per = get(map, 1, name);
	let mut min = max_ore / ore_per;
	let mut max = min * 2;
	while min < max-1 {
		let fuel = (min + max) / 2;
		reset(map);
		if get(map, fuel, name) > max_ore {
			max = fuel;
		} else {
			min = fuel;
		}
	}
	min
}

const TRIL: isize = 1000000000000;

#[test]
fn example1() {
	let mut keys = decode_recipes(
		"157 ORE => 5 NZVS
		165 ORE => 6 DCFZ
		44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
		12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
		179 ORE => 7 PSHF
		177 ORE => 5 HKGWZ
		7 DCFZ, 7 PSHF => 2 XJWVT
		165 ORE => 2 GPVTF
		3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT");
	assert_eq!(get(&mut keys, 1, "FUEL"), 13312);
	reset(&mut keys);
	assert_eq!(get_max(&mut keys, TRIL, "FUEL"), 82892753);
}
