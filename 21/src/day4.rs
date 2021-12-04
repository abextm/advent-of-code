#[aoc(day4, part1)]
fn day4_part1(input: &str) -> u32 {
	let mut bits = input.trim().split("\n\n");
	let numbers: Vec<u32> = bits.next().unwrap().split(",").map(|x| x.parse().unwrap()).collect();
	let mut boards: Vec<Vec<Vec<u32>>> = bits.map(|b| 
		b.split("\n").map(|l| 
			l.split(" ")
				.filter(|x|x.len() > 0)
				.map(|x| x.parse().unwrap())
				.collect()
			).collect()
		).collect();
	
		const DONE: u32 = 0xFFFF_FFFF;

	for test in numbers {
		for board in boards.iter_mut() {
			let mut done = false;
			for line in board.iter_mut() {
				for n in line.iter_mut() {
					if *n == test {
						*n = DONE;
					}
				}
			}

			for line in board.iter() {
				done |= line.iter().all(|&x| x == DONE);
			}
			for col in 0..5 {
				done |= board.iter().all(|l| l[col] == DONE);
			}
			if done {
				let sum: u32 = board.iter().map(|l| l.iter().filter(|&&x| x != DONE).sum::<u32>()).sum();
				return sum * test;
			}
		}
	}

	panic!();
}
#[aoc(day4, part2)]
fn day4_part2(input: &str) -> u32 {
	let mut bits = input.trim().split("\n\n");
	let numbers: Vec<u32> = bits.next().unwrap().split(",").map(|x| x.parse().unwrap()).collect();
	let mut boards: Vec<Vec<Vec<u32>>> = bits.map(|b| 
		b.split("\n").map(|l| 
			l.split(" ")
				.filter(|x|x.len() > 0)
				.map(|x| x.parse().unwrap())
				.collect()
			).collect()
		).collect();
	
		const DONE: u32 = 0xFFFF_FFFF;

	let mut done: Vec<bool> = boards.iter().map(|_| false).collect();
	for test in numbers {
		for (id, board) in boards.iter_mut().enumerate() {
			if done[id] {
				continue;
			}
			for line in board.iter_mut() {
				for n in line.iter_mut() {
					if *n == test {
						*n = DONE;
					}
				}
			}

			for line in board.iter() {
				done[id] |= line.iter().all(|&x| x == DONE);
			}
			for col in 0..5 {
				done[id] |= board.iter().all(|l| l[col] == DONE);
			}
			if done.iter().all(|&x|x) {
				let sum: u32 = board.iter().map(|l| l.iter().filter(|&&x| x != DONE).sum::<u32>()).sum();
				return sum * test;
			}
		}
	}

	panic!();
}