type Node = [u8; 3];
use std::collections::HashMap;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
	let mut input = input.lines();
	let mut directions = input.next().unwrap().as_bytes().iter().cycle();
	input.next();
	
	let nodes = input.map(|l| {
		let l = l.as_bytes();
		(l[0..3].try_into().unwrap(), [l[7..10].try_into().unwrap(), l[12..15].try_into().unwrap()])
	}).collect::<HashMap<Node, [Node; 2]>>();

	
	let mut node: Node = "AAA".as_bytes().try_into().unwrap();
	let target: Node = "ZZZ".as_bytes().try_into().unwrap();

	let mut steps = 0;
	while node != target {
		let index = match directions.next().unwrap() {
			b'L' => 0,
			b'R' => 1,
			v => panic!("{:?}", *v as char)
		};
		node = nodes.get(&node).unwrap()[index];
		steps += 1;
	}

	steps
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
	let mut input = input.lines();
	let mut directions = input.next().unwrap().as_bytes().iter().cycle();
	input.next();
	
	let nodes = input.map(|l| {
		let l = l.as_bytes();
		(l[0..3].try_into().unwrap(), [l[7..10].try_into().unwrap(), l[12..15].try_into().unwrap()])
	}).collect::<HashMap<Node, [Node; 2]>>();

	nodes.keys()
		.filter(|n| n.last() == Some(&b'A'))
		.map(|node| {
			let mut node = *node;
			let mut steps = 0;
			while node[2] != b'Z' {
				let index = match directions.next().unwrap() {
					b'L' => 0,
					b'R' => 1,
					v => panic!("{:?}", *v as char)
				};
				node = nodes.get(&node).unwrap()[index];
				steps += 1;
			}
			println!("{}", steps);
			steps
		})
		.reduce(num::integer::lcm)
		.unwrap()
}