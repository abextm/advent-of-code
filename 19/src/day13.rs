use crate::taken::TakeN;
use crate::vm;
use std::collections::HashSet;

mod tile {
	pub const EMPTY: i64 = 0;
	pub const WALL: i64 = 1;
	pub const BLOCK: i64 = 2;
	pub const H_PADDLE: i64 = 3;
	pub const BALL: i64 = 4;
}

#[aoc(part1=296)]
fn day13_part1(input: &str) -> usize {
	let mut vm = vm::new_from_str(input).unwrap().map(|x| x.unwrap());
	let mut set = HashSet::new();
	while let Some(v) = vm.take_n() {
		let [x, y, tile_id]: [_; 3] = v;
		if tile_id == tile::BLOCK {
			set.insert((x, y));
		}
	}
	set.len()
}

#[aoc(part2=13824)]
fn day13_part2(input: &str) -> i64 {
	let mut vm = vm::new_from_str(input)
		.unwrap()
		.with_input(std::iter::repeat(0));
	vm.memory[0] = 2;
	let mut ball = 0;
	let mut paddle = 0;
	let mut score = 0;
	loop {
		if let Some(v) = vm.take_n() {
			let [x, y, tile_id]: [_; 3] = v;
			let (x, y, tile_id) = (x.unwrap(), y.unwrap(), tile_id.unwrap());
			if (x, y) == (-1, 0) {
				score = tile_id;
			} else {
				match tile_id {
					tile::H_PADDLE => paddle = x,
					tile::BALL => {ball = x; },
					_ => (),
				}
			}
		} else {
			break
		}
		vm = vm.with_input(std::iter::repeat(match ball.cmp(&paddle) {
			std::cmp::Ordering::Less => -1,
			std::cmp::Ordering::Equal => 0,
			std::cmp::Ordering::Greater => 1,
		}));
	}
	score
}