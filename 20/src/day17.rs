use crate::grid;

#[aoc(day17, part1)]
fn day17_part1(input: &str) -> usize {
	let mut state = std::collections::HashSet::new();
	for (x, y, v) in grid::Grid::new(input).iter() {
		if v == '#' as u8 {
			state.insert((x as isize, y as isize, 0 as isize));
		}
	}

	let dirs = (-1..=1)
		.flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (x, y, z))))
		.filter(|&(x, y, z)| !(x == 0 && y == 0 && z == 0))
		.collect::<Vec<(isize, isize, isize)>>();

	for _ in 0..6 {
		let mut next_state = state.clone();
		for p in state.iter() {
			let mut active = 0;
			for d in dirs.iter() {
				let tp = add3(p, d);
				if state.contains(&tp) {
					active += 1;
				} else {
					let mut active = 0;
					for d in dirs.iter() {
						let itp = add3(&tp, d);
						if state.contains(&itp) {
							active += 1;
						}
					}
					if active == 3 {
						next_state.insert(tp);
					}
				}
			}
			if !(active == 2 || active == 3) {
				next_state.remove(p);
			}
		}
		state = next_state;
	}

	state.len()
}

fn add3(a: &(isize, isize, isize), b: &(isize, isize, isize)) -> (isize, isize, isize) {
	(a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

#[aoc(day17, part2)]
fn day17_part2(input: &str) -> usize {
	let mut state = std::collections::HashSet::new();
	for (x, y, v) in grid::Grid::new(input).iter() {
		if v == '#' as u8 {
			state.insert((x as isize, y as isize, 0 as isize, 0 as isize));
		}
	}

	let dirs = (-1..=1)
		.flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).flat_map(move |z| (-1..=1).map(move |w| (x, y, z, w)))))
		.filter(|&(x, y, z, w)| !(x == 0 && y == 0 && z == 0 && w == 0))
		.collect::<Vec<(isize, isize, isize, isize)>>();

	for _ in 0..6 {
		let mut next_state = state.clone();
		for p in state.iter() {
			let mut active = 0;
			for d in dirs.iter() {
				let tp = add4(p, d);
				if state.contains(&tp) {
					active += 1;
				} else {
					let mut active = 0;
					for d in dirs.iter() {
						let itp = add4(&tp, d);
						if state.contains(&itp) {
							active += 1;
						}
					}
					if active == 3 {
						next_state.insert(tp);
					}
				}
			}
			if !(active == 2 || active == 3) {
				next_state.remove(p);
			}
		}
		state = next_state;
	}

	state.len()
}

fn add4(a: &(isize, isize, isize, isize), b: &(isize, isize, isize, isize)) -> (isize, isize, isize, isize) {
	(a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}
