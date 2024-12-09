use std::collections::VecDeque;

struct MapIter {
	map: Vec<u8>,
	// id, rem
	lhs_index: (usize, u8),
	rhs_index: (usize, u8),
}

impl MapIter {
	fn new(map: Vec<u8>) -> MapIter {
		MapIter {
			lhs_index: (0, 0),
			rhs_index: (map.len() - 1, map[map.len() - 1] - 1),
			map,
		}
	}
	fn take_left(&mut self) -> Option<usize> {
		if self.lhs_index > self.rhs_index {
			return None;
		}
		let ret = self.lhs_index.0;
		let ret = if ret & 1 == 0 {
			Some(ret / 2)
		} else {
			None
		};
		self.lhs_index.1 += 1;
		while self.lhs_index.1 >= self.map[self.lhs_index.0] {
			self.lhs_index.1 = 0;
			self.lhs_index.0 += 1;
		}
		ret
	}
	fn take_right(&mut self) -> Option<usize> {
		loop {
			if self.lhs_index > self.rhs_index {
				return None;
			}

			let ret = self.rhs_index.0;

			while self.rhs_index.1 == 0 {
				self.rhs_index.0 -= 1;
				self.rhs_index.1 = self.map[self.rhs_index.0];
			}
			self.rhs_index.1 -= 1;

			if ret & 1 == 0 {
				return Some(ret / 2);
			} else {
				continue;
			};
		}
	}
}


impl Iterator for MapIter {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(v) = self.take_left() {
			Some(v)
		} else {
			self.take_right()
		}
	}
}

#[aoc(part1 = 6344673854800)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let map = MapIter::new(input.trim().bytes().map(|n| n - b'0').collect());
	//map.inspect(|v| print!("{} ", v)).take(100).enumerate().map(|(i, v)| i * v).sum::<usize>()
	map.enumerate().map(|(i, v)| i * v).sum::<usize>()
}

#[aoc(part2=6360363199987)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let mut map: VecDeque<(usize, [u8; 2])> = input.trim().bytes().map(|n| n - b'0').chain([0].into_iter()).array_chunks().enumerate().collect();

	let mut i = map.len() - 1;
	loop {
		let (file_id, [size, empty]) = map[i];

		if let Some(i2) = map.iter().take(i).position(|(_file_id2, [_size2, empty2])| *empty2 >= size) {
			let (_file_id2, [_size2, empty2]) = map[i2];
			map[i] = (0, [0, size + empty]);
			map[i2].1[1] = 0;
			map.insert(i2 + 1, (file_id, [size, empty2 - size]));
			i += 1;
		}

		if i == 0 {
			break;
		}
		i -= 1;
	}

	let mut checksum = 0;
	let mut block_id = 0;

	for (file_id, [v, empty]) in map.iter() {
		for i in 0..*v {
			checksum += file_id * block_id;
			block_id += 1;
		}
		block_id += *empty as usize;
	}

	checksum
}

#[aoc(part1 = 1928, part2 = 2858)]
const EX2: &str = "2333133121414131402";
