use smallvec::SmallVec;

type Point = [isize; 3];

#[derive(Debug)]
struct Brick {
	low: Point,
	hi: Point,

	below_count: usize,
	above: SmallVec<[u16; 4]>,
}

fn parse_bricks(input: &str) -> Vec<Brick> {
	let mut xy_max = [0isize; 2];
	let mut bricks = input.lines()
		.map(|line| {
			let [low, hi] = line.split("~")
				.map(|coord| coord.split(",").map(|c| c.parse::<isize>().expect(c)).array_chunks::<3>().next().unwrap())
				.array_chunks::<2>().next().unwrap();
			xy_max[0] = xy_max[0].max(hi[0]);
			xy_max[1] = xy_max[1].max(hi[1]);
			Brick {
				low,
				hi,

				below_count: 0,
				above: SmallVec::new(),
			}
		})
		.collect::<Vec<_>>();

	bricks.sort_unstable_by_key(|b| b.low[2]);

	let grid_width = xy_max[0] + 1;
	let mut grid = vec![(0, u16::MAX); (grid_width * (xy_max[1] + 1)) as usize];

	let mut touching_bricks = Vec::<u16>::new();

	for brick_id in 0..bricks.len() {
		touching_bricks.clear();
		let Brick{low, hi, ..} = bricks[brick_id];

		let brick_iter = (low[1]..=hi[1])
			.flat_map(|y| (low[0]..=hi[0]).map(move |x| (x + y * grid_width) as usize));
		
		let max_hi = brick_iter.clone().map(|pt| grid[pt].0).max().unwrap();
		
		let brick = &mut bricks[brick_id];
		let delta = brick.low[2] - (max_hi + 1);
		brick.low[2] -= delta;
		brick.hi[2] -= delta;
		let top = brick.hi[2];

		for pt in brick_iter {
			let (ihi, ibrick_id) = grid[pt];
			if max_hi == ihi && ibrick_id != u16::MAX && !touching_bricks.contains(&ibrick_id) {
				touching_bricks.push(ibrick_id);
			}
			grid[pt] = (top, brick_id as u16);
		}

		brick.below_count = touching_bricks.len();

		for &inner_brick_id in touching_bricks.iter() {
			bricks[inner_brick_id as usize].above.push(brick_id as u16);
		}
	}

	bricks
}

#[aoc(day22, part1)]
fn part1(input: &str) -> usize {
	let bricks = parse_bricks(input);
	bricks.iter().filter(|br| {
		br.above.iter().all(|&bid| bricks[bid as usize].below_count > 1)
	}).count()
}

fn fall_brick(bricks: &[Brick], supports: &mut [u8], todo: &mut Vec<usize>, index: usize) -> usize {
	let mut sum = 0;
	todo.push(index);
	while let Some(index) = todo.pop() {
		for &b in bricks[index].above.iter() {	
			let supp = supports[b as usize] - 1;
			supports[b as usize] = supp;
			if supp == 0 {
				sum += 1;
				todo.push(b as usize);
			}
		}
	}

	sum
}

#[aoc(day22, part2)]
fn part2(input: &str) -> usize {
	let bricks = parse_bricks(input);
	let supports_template = bricks.iter()
		.map(|b| b.below_count as u8)
		.collect::<Vec<_>>();
	let mut supports = supports_template.clone();
	let mut todo = Vec::new();
	(0..bricks.len()).map(|id| {
		if id != 0 {
			supports.copy_from_slice(&supports_template);
		}
		let v = fall_brick(&bricks, &mut supports, &mut todo, id);
		v
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
	assert_eq!(7, part2(EXAMPLE));
}