#[aoc(day10, part1)]
fn day10_part1(input: &str) -> usize {
	let mut adapters = input
		.trim()
		.split("\n")
		.map(|s| s.parse().unwrap())
		.collect::<Vec<i64>>();
	let wanted = adapters.iter().max().unwrap() + 3;
	adapters.push(wanted);
	adapters.push(0);
	adapters.sort();

	let diff = adapters
		.iter()
		.zip(adapters.iter().skip(1))
		.map(|(a, b)| b - a)
		.collect::<Vec<_>>();
	println!("{:?}", diff);

	diff.iter().filter(|&&x| x == 1).count() * diff.iter().filter(|&&x| x == 3).count()
}

#[test]
fn testp1() {
	let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
	assert_eq!(day10_part1(input), 7 * 5);
}

#[aoc(day10, part2)]
fn day10_part2(input: &str) -> f64 {
	let mut adapters = input
		.trim()
		.split("\n")
		.map(|s| s.parse().unwrap())
		.collect::<Vec<i64>>();
	let wanted = adapters.iter().max().unwrap() + 3;
	adapters.push(wanted);
	adapters.push(0);
	adapters.sort();

	let mut per_adapter_mutations = vec![0.0; adapters.len()];
	per_adapter_mutations[0] = 1.0;
	for i in 0..adapters.len() {
		let val = adapters[i];
		let permutations = per_adapter_mutations[i];
		for ii in (i + 1)..adapters.len() {
			let ival = adapters[ii];
			if ival - 3 > val {
				break;
			}

			per_adapter_mutations[ii] += permutations;
		}
	}
	*per_adapter_mutations.last().unwrap()
}

#[test]
fn testp2() {
	let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
	assert_eq!(day10_part2(input), 8.0);
}
