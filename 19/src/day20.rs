use std::collections::hash_map::Entry;
use std::collections::HashMap;
use crate::dijkstra::dijkstra;
use crate::grid::Grid;

#[aoc(part1=432)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let grid = Grid::from_char_grid(input);

	let mut points = HashMap::new();

	let mut connections = HashMap::new();

	for (x, y, v) in grid.iter() {
		if *v >= b'A' && *v <= b'Z' {
			if let Some(dot) = grid.adjacent4(x, y).filter(|(_, _, &x)| x == b'.').next() {
				if let Some(other) = grid.adjacent4(x, y).filter(|(_, _, &x)| x >= b'A' && x <= b'Z').next() {
					let key = if x < other.0 || y < other.1 {
						[*v, *other.2]
					} else {
						[*other.2, *v]
					};
					let pt = (dot.0, dot.1);
					match points.entry(key) {
						Entry::Occupied(entry) => {
							let other_pt = entry.remove_entry().1;
							connections.insert(pt, other_pt);
							connections.insert(other_pt, pt);
						},
						Entry::Vacant(entry) => {
							entry.insert(pt);
						},
					}
				}
			}
		}
	}

	assert_eq!(points.len(), 2);

	let start = *points.get(&[b'A'; 2]).unwrap();
	let end = *points.get(&[b'Z'; 2]).unwrap();

	let connections = &connections;
	dijkstra(start, Grid::blank(&grid, None), |&p| p == end, |pt| {
		grid.adjacent4(pt.0, pt.1).filter_map(move |(x, y, &v)| {
			if v == b'.' {
				Some(((x, y), 1))
			} else if v >= b'A' && v <= b'Z' {
				connections.get(&pt).map(|pt| (*pt, 1))
			} else {
				None
			}
		})
	}).unwrap()
}

#[aoc(part2=5214)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let grid = Grid::from_char_grid(input);

	let mut points = HashMap::new();

	let mut connections = HashMap::new();

	for (x, y, v) in grid.iter() {
		if *v >= b'A' && *v <= b'Z' {
			if let Some(dot) = grid.adjacent4(x, y).filter(|(_, _, &x)| x == b'.').next() {
				if let Some(other) = grid.adjacent4(x, y).filter(|(_, _, &x)| x >= b'A' && x <= b'Z').next() {
					let key = if x < other.0 || y < other.1 {
						[*v, *other.2]
					} else {
						[*other.2, *v]
					};
					let pt = (dot.0, dot.1);
					match points.entry(key) {
						Entry::Occupied(entry) => {
							let other_pt = entry.remove_entry().1;
							let dir = if pt.0 <= 2 || pt.1 <= 2 || pt.0 >= grid.width() - 3 || pt.1 >= grid.height() - 3 {
								-1
							} else {
								1
							};
							connections.insert(pt, (other_pt, dir));
							connections.insert(other_pt, (pt, -dir));
						},
						Entry::Vacant(entry) => {
							entry.insert(pt);
						},
					}
				}
			}
		}
	}

	assert_eq!(points.len(), 2);

	let start = *points.get(&[b'A'; 2]).unwrap();
	let start = (start, 0);
	let end = *points.get(&[b'Z'; 2]).unwrap();
	let end = (end, 0);

	let connections = &connections;
	dijkstra(start, HashMap::new(), |&p| p == end, |(pt, level)| {
		grid.adjacent4(pt.0, pt.1).filter_map(move |(x, y, &v)| {
			if v == b'.' {
				Some((((x, y), level), 1))
			} else if v >= b'A' && v <= b'Z' {
				if let Some((npt, dir)) = connections.get(&pt) {
					if level + dir < 0 {
						None
					} else {
						Some(((*npt, (level + dir)), 1))
					}
				} else {
					None
				}
			} else {
				None
			}
		})
	}).unwrap()
}