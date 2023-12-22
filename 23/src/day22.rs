type Point = [isize; 3];

#[derive(Debug)]
struct Brick {
	low: Point,
	hi: Point,

	below: Vec<usize>,
	above: Vec<usize>,
}

fn parse_bricks(input: &str) -> Vec<Brick> {
	//let mut max_height = 0;
	let mut bricks = input.lines()
		.map(|line| {
			let coords = line.split("~")
				.map(|coord| coord.split(",").map(|c| c.parse::<isize>().expect(c)).array_chunks::<3>().next().unwrap())
				.array_chunks::<2>().next().unwrap();
			//max_height = max_height.max(coords[1][2] - coords[0][2]);
			Brick {
				low: coords[0],
				hi: coords[1],

				below: Vec::new(),
				above: Vec::new(),
			}
		})
		.collect::<Vec<_>>();

	bricks.sort_by_key(|b| b.low[2]);

	let mut touching_bricks = Vec::<usize>::new();

	for brick_id in 0..bricks.len() {
		let brick = &bricks[brick_id];

		touching_bricks.clear();
		let mut max_hi = 0;

		for inner_brick_id in (0..brick_id).rev() {
			let ibrick = &bricks[inner_brick_id];
			if ibrick.hi[2] < max_hi || ibrick.low[2] > brick.low[2] {
				continue;
			}
			let outside = brick.low[0] > ibrick.hi[0] || brick.hi[0] < ibrick.low[0]
				|| brick.low[1] > ibrick.hi[1] || brick.hi[1] < ibrick.low[1];
			if outside {
				continue;
			}

			if ibrick.low[2] == brick.low[2] {
				panic!("{:?} {:?}", brick, ibrick)
			}

			if ibrick.hi[2] > max_hi {
				max_hi = ibrick.hi[2];
				touching_bricks.clear();
			}
			if ibrick.hi[2] == max_hi {
				touching_bricks.push(inner_brick_id);
			}
		}

		for &inner_brick_id in touching_bricks.iter() {
			bricks[inner_brick_id].above.push(brick_id);
		}

		let brick = &mut bricks[brick_id];
		let delta = brick.low[2] - (max_hi + 1);
		brick.low[2] -= delta;
		brick.hi[2] -= delta;

		assert_eq!(brick.above.len(), 0);
		assert_eq!(brick.below.len(), 0);

		std::mem::swap(&mut touching_bricks, &mut brick.below);
	}

	bricks
}

#[aoc(day22, part1)]
fn part1(input: &str) -> usize {
	let bricks = parse_bricks(input);
	bricks.iter().filter(|br| {
		br.above.iter().all(|&bid| bricks[bid].below.len() > 1)
	}).count()
}

fn count_fallen(bricks: &[Brick], fallen: &mut [bool], index: usize) -> usize {
	if fallen[index] {
		return 0;
	}
	let brick = &bricks[index];
	if brick.below.iter().all(|&ib| fallen[ib]) {
		fall_brick(bricks, fallen, index)
	} else {
		0
	}
}

fn fall_brick(bricks: &[Brick], fallen: &mut [bool], index: usize) -> usize {
	fallen[index] = true;
	let mut sum = 1;
	for &b in bricks[index].above.iter() {
		sum += count_fallen(bricks, fallen, b);
	}

	sum
}

#[aoc(day22, part2)]
fn part2(input: &str) -> usize {
	let bricks = parse_bricks(input);
	let mut fallen = vec![false; bricks.len()];
	(0..bricks.len()).map(|id| {
		fallen.fill(false);
		fall_brick(&bricks, &mut fallen, id) - 1
	}).sum()
}


#[cfg(test)]
const EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

#[test]
fn test_p1() {
	assert_eq!(5, part1(EXAMPLE));
}

#[test]
fn test_p2() {
	assert_eq!(70, part2(EXAMPLE));
}