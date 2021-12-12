use std::collections::{HashMap, hash_map::Entry};

const NONE: usize = usize::MAX;

struct Graph<'a> {
	nodes: HashMap<&'a str, usize>,
	large: Vec<bool>,
	edges: Vec<usize>,
}

impl<'a> Graph<'a> {
	fn parse(input: &'a str) -> Graph<'a> {
		let mut nodes: HashMap<&str, usize> = HashMap::new();
		let mut large = Vec::new();
		for line  in input.lines() {
			for node in line.split("-") {
				let e = nodes.entry(node);
				if let Entry::Vacant(u) = e {
					u.insert(large.len());
					large.push(node.chars().next().unwrap().is_uppercase());
				}
			}
		}
		let num_nodes = large.len();
		let mut edges = vec![NONE; nodes.len() * nodes.len()];
		for line  in input.lines() {
			let mut i = line.split("-").map(|v| *nodes.get(v).expect(v));
			let a = i.next().unwrap();
			let b = i.next().unwrap();
			edges[a * num_nodes + b] = b;
			edges[b * num_nodes + a] = a;
		}
		Graph{nodes, large, edges}
	}

	fn len(&self) -> usize {
		self.large.len()
	}

	fn get(&self, name: &str) -> usize {
		*self.nodes.get(name).unwrap()
	}

	fn count_paths(&self, reuse: bool) -> usize {
		let mut visited = vec![0; self.len()];
		let start = self.get("start");
		self.count_paths0(&mut visited, !reuse, start, start, self.get("end"))
	}

	fn count_paths0(&self, visited: &mut [u32], mut small_is_used: bool, true_start: usize, from: usize, to: usize) -> usize {
		if from == to {
			return 1
		}

		let big = self.large[from];
		let vis = visited[from];
		if !big && vis > 0 {
			if small_is_used || from == true_start {
				return 0;
			}
			small_is_used = true
		}
		visited[from] += 1;

		let v = (0..self.len()).map(|edge| {
			let other = self.edges[(from * self.len()) + edge];
			if other == NONE {
				return 0;
			}

			self.count_paths0(visited, small_is_used, true_start, other, to)
		}).sum::<usize>();
		visited[from] -= 1;
		v
	}
}

#[aoc(day12, part1)]
fn day12_part1(input: &str) -> usize {
	Graph::parse(input).count_paths(false)
}
#[aoc(day12, part2)]
fn day12_part2(input: &str) -> usize {
	Graph::parse(input).count_paths(true)
}

#[test]
fn p1_sm() {
	assert_eq!(10, day12_part1("start-A
start-b
A-c
A-b
b-d
A-end
b-end"));
}

#[test]
fn p2_mid() {
	assert_eq!(103, day12_part2("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"));
}