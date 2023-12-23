use crate::grid::Grid;
use std::collections::HashMap;
use smallvec::SmallVec;

type Index = usize;
type Point = [usize; 2];

#[derive(Debug)]
struct Link {
	target: Index,
	distance: usize,
	direction: Point,
	available: bool,
}

struct Vert {
	edges: SmallVec<[Link; 4]>,
}

fn parse(input: &str) -> (Index, Index, Vec<Vert>) {
	let g = Grid::from_char_grid(input);

	let start = [1, 0];
	let end = [g.width() - 2, g.height() - 1];

	let mut verts_by_point = HashMap::<Point, Index>::new();
	let mut verts = Vec::<Vert>::new();
	let mut todo = Vec::new();
	todo.push(start);

	while let Some(pt) = todo.pop() {
		let vert_index = *verts_by_point.entry(pt)
			.or_insert_with(|| {
				verts.push(Vert {
					edges: SmallVec::new(),
				});
				verts.len() - 1
			});
		let vert = &verts[vert_index];

		let edges = g.adjacent4(pt[0], pt[1])
			.filter(|&(x, y, &v)| v != b'#' && vert.edges.iter().find(|pt| pt.direction == [x, y]).is_none())
			.collect::<SmallVec<[_; 3]>>();

		edges.into_iter()
			.for_each(|(x, y, _)| {
				let mut last_pt = pt;
				let mut pt = [x, y];
				let mut distance = 0;
				let mut dir_mask = 0b11;
				loop {
					let mut adj = g.adjacent4(pt[0], pt[1])
						.filter(|&(x, y, &v)| v != b'#' && (x != last_pt[0] || y != last_pt[1]));
					let next = adj.next();
					let rem = adj.next();
					if next == None || rem != None {
						let inner_vert_index = *verts_by_point.entry(pt)
							.or_insert_with(|| {
								verts.push(Vert {
									edges: SmallVec::new(),
								});
								verts.len() - 1
							});

						verts[inner_vert_index].edges.push(Link {
							target: vert_index,
							distance,
							direction: last_pt,
							available: dir_mask & 0b01 != 0,
						});
						verts[vert_index].edges.push(Link {
							target: inner_vert_index,
							distance,
							direction: [x, y],
							available: dir_mask & 0b10 != 0,
						});

						todo.push(pt);
						return;
					}

					let next = next.unwrap();

					last_pt = pt;
					pt = [next.0, next.1];
					distance += 1;

					if let Some(bit) = match *next.2 {
						b'.' => None,
						b'>' => Some(last_pt[0] > pt[0]),
						b'<' => Some(last_pt[0] < pt[0]),
						b'^' => Some(last_pt[1] < pt[1]),
						b'v' => Some(last_pt[1] > pt[1]),
						_ => panic!(),
					} {
						dir_mask &= !(1 << (bit as usize));
					}
				}
			});
	}

	let start = verts_by_point[&start];
	let end = verts_by_point[&end];
	
	/*
	println!("{} [label=\"start\"];", start);
	println!("{} [label=\"end\"];", end);

	for (vid, vert) in verts.iter().enumerate() {
		for other in vert.edges.iter() {
			if other.available {
				println!("{} -> {} [label=\"{}\"];", vid, other.target, other.distance);
			}
		}
	}
	// */

	(start, end, verts)
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
	let (start, end, verts) = parse(input);
	walk_recursive(&verts, start, end)
}

fn walk_recursive(verts: &[Vert], pt: Index, end: Index) -> usize {
	if pt == end {
		return 0;
	}
	verts[pt].edges.iter().map(|edge|{
		if !edge.available {
			return 0;
		}
		edge.distance + walk_recursive(verts, edge.target, end)
	}).max().unwrap() + 1
}

#[aoc(day23, part2)]
fn part2(input: &str) -> usize {
	let (start, end, verts) = parse(input);
	let mut visited = vec![false; verts.len()];
	walk_recursive_noback(&verts, &mut visited, start, end).unwrap()
}

fn walk_recursive_noback(verts: &[Vert], visited: &mut [bool], pt: Index, end: Index) -> Option<usize> {
	if pt == end {
		return Some(0);
	}
	if visited[pt] {
		return None;
	}
	visited[pt] = true;
	let res = verts[pt].edges.iter().filter_map(|edge| {
		walk_recursive_noback(verts, visited, edge.target, end).map(|v| edge.distance + v)
	}).max().map(|v| v + 1);
	visited[pt] = false;
	res
}

const EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

#[test]
fn test_p1() {
	assert_eq!(94, part1(EXAMPLE))
}
#[test]
fn test_p2() {
	assert_eq!(154, part2(EXAMPLE))
}