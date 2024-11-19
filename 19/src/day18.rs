use std::collections::HashMap;
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};
use crate::dijkstra::dijkstra;
use crate::grid::Grid;

mod tile {
	pub const STRUCTURE: u8 = '#' as u8;
	pub const ENTRY: u8 = '@' as u8;
	pub const AIR: u8 = '.' as u8;
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Id<N>(pub N);

fn id<N: TryInto<usize> + TryFrom<usize, Error: Debug>>(v: usize) -> Id<N> {
	Id(v.try_into().unwrap())
}

impl<N: TryInto<usize, Error: Debug>, T> Index<Id<N>> for [T] {
	type Output = T;

	fn index(&self, index: Id<N>) -> &Self::Output {
		self.index(index.0.try_into().unwrap())
	}
}

impl<N: TryInto<usize, Error: Debug>, T> IndexMut<Id<N>> for [T] {
	fn index_mut(&mut self, index: Id<N>) -> &mut Self::Output {
		self.index_mut(index.0.try_into().unwrap())
	}
}

impl<N: TryInto<usize, Error: Debug>, T> Index<Id<N>> for Vec<T> {
	type Output = T;

	fn index(&self, index: Id<N>) -> &Self::Output {
		self.index(index.0.try_into().unwrap())
	}
}

impl<N: TryInto<usize, Error: Debug>, T> IndexMut<Id<N>> for Vec<T> {
	fn index_mut(&mut self, index: Id<N>) -> &mut Self::Output {
		self.index_mut(index.0.try_into().unwrap())
	}
}

impl<N> From<N> for Id<N> {
	fn from(value: N) -> Self {
		Id(value)
	}
}


#[derive(Debug)]
struct Node {
	char: u8,
	index: Id<u16>,
	x: u8,
	y: u8,
	edges: Vec<Edge>,
}

#[derive(Debug, Copy, Clone)]
struct Edge {
	node: Id<u16>,
	dist: u16,
	doors: u32,
}

struct NodeBuilder<'a> {
	g: Grid<&'a [u8]>,
	nodes: Vec<Node>,
	ids: Grid<Vec<Id<u16>>>,
	todo: Vec<((u8, u8), (u8, u8))>,
	tmp: Vec<(u8, u8)>,
}

impl<'a> NodeBuilder<'a> {
	fn build(g: Grid<&[u8]>) -> Vec<Node> {
		let mut nb = NodeBuilder {
			g,
			nodes: Vec::new(),
			ids: Grid::blank(&g, u16::MAX.into()),
			todo: Vec::new(),
			tmp: Vec::with_capacity(4),
		};

		// create start points
		for (x, y, _) in g.filter_enumerate(|&n| n == tile::ENTRY) {
			let pt = (x.try_into().unwrap(), y.try_into().unwrap());
			nb.edges(pt, (u8::MAX, u8::MAX));
			nb.create_node(pt);
		}

		fn node_mask(node: &Node) -> u32 {
			if node.char >= b'A' && node.char <= b'Z' {
				1 << node.char - b'A'
			} else {
				0
			}
		}

		// find all keys/doors/splits
		while let Some((mut next, mut last)) = nb.todo.pop() {
			let start = nb.ids[last];
			let mut dist = 0;
			loop {
				dist += 1;
				nb.edges(next, last);
				if nb.tmp.len() > 1 || nb.g[next] != tile::AIR {
					nb.create_node(next);
					if !nb.nodes[start].edges.iter().any(|e| e.node == nb.ids[next]) {
						let doors = node_mask(&nb.nodes[start]) | node_mask(&nb.nodes[nb.ids[next]]);
						nb.nodes[start].edges.push(Edge {
							node: nb.ids[next],
							dist,
							doors,
						});
						nb.nodes[nb.ids[next]].edges.push(Edge {
							node: start,
							dist,
							doors,
						});
					}
				} else if nb.tmp.len() == 1 {
					last = next;
					next = nb.tmp[0];
					continue;
				}
				break;
			}
		}

		// remove any dead ends recursively
		for i in 0..nb.nodes.len() {
			nb.maybe_trim(id(i));
		}

		// fully link the graph, removing doors and splits
		for b_index in 0..nb.nodes.len() {
			let b_index: Id<u16> = id(b_index);
			let node = &mut nb.nodes[b_index];
			if node.edges.len() == 0 {
				continue;
			}

			let can_remove = !(node.char == b'@' || (node.char >= b'a' && node.char <= b'z'));

			let edges = if can_remove {
				let mut edges = Vec::new();
				std::mem::swap(&mut edges, &mut node.edges);
				edges
			} else {
				node.edges.clone()
			};

			for &ba in edges.iter() {
				let a_edges = &mut nb.nodes[ba.node].edges;
				if can_remove {
					let index = a_edges.iter().position(|e| e.node == b_index && e.doors == ba.doors).unwrap();
					a_edges.swap_remove(index);
				}
				for &bc in edges.iter() {
					if ba.node != bc.node {
						let dist_ac = ba.dist + bc.dist;
						let doors_ac = ba.doors | bc.doors;
						match a_edges.iter().position(|e| e.node == bc.node) {
							Some(index) => {
								if a_edges[index].dist > dist_ac {
									if a_edges[index].doors != doors_ac {
										dbg!(a_edges[index].doors, doors_ac, a_edges[index].doors ^ doors_ac, &nb.nodes[ba.node], &nb.nodes[bc.node]);
										panic!();
									}
									a_edges[index].dist = dist_ac;
								}
							}
							None => a_edges.push(Edge {
								node: bc.node,
								dist: dist_ac,
								doors: doors_ac,
							}),
						}
					}
				}
			}
		}

		// garbage collect
		let mut last_index = 0;
		for n in nb.nodes.iter_mut() {
			if n.char >= b'a' && n.char <= b'z' {
				n.index = Id((n.char - b'a').into());
				last_index = last_index.max(n.index.0);
			}
		}
		for n in nb.nodes.iter_mut() {
			if n.edges.len() > 0 && !(n.char >= b'a' && n.char <= b'z') {
				last_index += 1;
				n.index = Id(last_index);
			}
		}

		let mut out = Vec::with_capacity(last_index as usize + 1);

		for node in nb.nodes.iter() {
			if node.edges.len() > 0 {
				out.spare_capacity_mut()[node.index] = MaybeUninit::new(Node {
					edges: node.edges.iter()
						.map(|e| Edge {
							node: nb.nodes[e.node].index,
							..*e
						})
						.collect(),
					..*node
				});
			}
		}

		unsafe { out.set_len(last_index as usize + 1) };

		out
	}

	fn maybe_trim(&mut self, index: Id<u16>) {
		let node = &mut self.nodes[index];
		if node.char == tile::AIR && node.edges.len() == 1 {
			let rem = node.edges[0].node;
			node.edges.clear();
			let edges = &mut self.nodes[rem].edges;
			let index = edges.iter().position(|&e| e.node == index).unwrap();
			edges.swap_remove(index);
			if edges.len() == 1 {
				self.maybe_trim(rem);
			}
		}
	}

	fn edges(&mut self, pt: (u8, u8), last: (u8, u8)) {
		self.tmp.clear();
		for (nx, ny, &v) in self.g.adjacent4(pt.0.into(), pt.1.into()) {
			if v != tile::STRUCTURE && !(nx == last.0.into() && ny == last.1.into()) {
				self.tmp.push((nx.try_into().unwrap(), ny.try_into().unwrap()));
			}
		}
	}

	fn create_node(&mut self, pt: (u8, u8)) -> bool {
		let id = &mut self.ids[pt];
		if id.0 != u16::MAX {
			return false;
		}

		let char = self.g[pt];

		id.0 = self.nodes.len().try_into().unwrap();
		self.nodes.push(Node {
			char,
			x: pt.0,
			y: pt.1,
			index: *id,
			edges: Vec::new(),
		});

		for &next in self.tmp.iter() {
			self.todo.push((next, pt))
		}

		true
	}
}

#[aoc(part1=3918)]
fn day18_part1(input: &str) -> usize {
	solve::<1>(input)
}

#[aoc(part2=2004)]
fn day18_part2(input: &str) -> usize {
	solve::<4>(input)
}

fn solve<const STARTS: usize>(input: &str) -> usize {
	let mut g = Grid::from_char_grid(input).map(|_, _, &x| x);

	if STARTS == 4 {
		let (x, y) = g.find(&b'@').unwrap();
		g[[x, y]] = b'#';
		for pt in crate::grid::adjacent8_points(&g, x, y) {
			g[pt] = if pt.0 == x || pt.1 == y {
				b'#'
			} else {
				b'@'
			};
		}
	}

	let nodes = NodeBuilder::build(g.as_ref());

	/*
	println!("dograph a {{");
	for node in nodes.iter() {
		println!("  {} [label=\"{} {} {} {}\"];", node.index.0, node.x, node.y, node.char as char, node.index.0);
		for edge in node.edges.iter() {
			println!("    {} -> {} [label=\"{:x} {}\"]", node.index.0, edge.node.0, edge.doors, edge.dist);
		}
	}
	println!("}}");
	 */

	let mut bitset = 0u32;
	for node in nodes.iter() {
		if node.char >= b'a' && node.char <= b'z' {
			bitset |= 1 << node.index.0;
		}
	}

	let starts: [Id<u8>; STARTS] = nodes.iter()
		.filter(|n| n.char == b'@')
		.map(|n| id(n.index.0 as usize))
		.array_chunks()
		.next()
		.unwrap();

	dijkstra((starts, bitset), HashMap::new(), |&(_pt, keys)| keys == 0, |(pts, keys)| {
		let nodes = &nodes;
		(0..pts.len()).flat_map(move |pt_i| {
			nodes[pts[pt_i]].edges.iter().filter_map(move |&e| {
				if (keys & e.doors) != 0 || keys & (1 << e.node.0) == 0 {
					return None;
				}
				let keys = keys & !(1 << e.node.0);
				let mut out = pts;
				out[pt_i] = Id(e.node.0 as u8);
				return Some(((out, keys), e.dist.into()));
			})
		})
	}).unwrap()
}

#[aoc(part2=72)]
const complex: &str = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############
";