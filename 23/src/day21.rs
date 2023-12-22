use crate::grid::Grid;

type Cell = u16;
const ROCK: Cell = Cell::MAX;
const UNVISITED: Cell = Cell::MAX - 1;
const START: Cell = Cell::MAX - 2;

struct State {
	g: Grid<Vec<Cell>>,
	start: (usize, usize),
	step: Cell,
	edge: Vec<(usize, usize)>,
}

impl State {
	fn parse(input: &str) -> State {
		let mut start = (0usize, 0usize);
		let g = Grid::from_char_grid(input).map(|x, y, v| match v {
			b'#' => ROCK,
			b'.' => UNVISITED,
			b'S' => {
				start = (x, y);
				START
			},
			_ => panic!(),
		});

		State {
			g,
			edge: Vec::new(),
			step: 0,
			start,
		}
	}

	fn simulate(&mut self, to_step: Cell) {
		if self.edge.len() == 0 {
			self.edge.push(self.start);
		}
		let mut next_edge = Vec::new();
		while self.step < to_step {
			self.step += 1;
			for pt in self.edge.iter() {
				for n in crate::grid::adjacent4_points(&self.g, pt.0, pt.1) {
					let cell = &mut self.g[n];
					if *cell == ROCK || *cell == self.step {
						continue;
					} else {
						*cell = self.step;
						next_edge.push(n);
					}
				}
			}
			std::mem::swap(&mut next_edge, &mut self.edge);
			next_edge.clear();
		}
	}
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
	let mut state = State::parse(input);
	state.simulate(64);
	state.edge.len()
}


#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
	let mut state = State::parse(input);

	let steps = 26501365;

	let repeats = 5;
	let mut grid = Grid::from_generator(&(state.g.width() * repeats, state.g.height() * repeats), |x, y| *state.g.get_wrapped(x as isize, y as isize));
	std::mem::swap(&mut state.g, &mut grid);
	state.start.0 += grid.width() * (repeats / 2);
	state.start.1 += grid.height() * (repeats / 2);

	state.simulate((steps % grid.width() + grid.width() * 2).try_into().unwrap());
	
	let mut counts = Grid::blank(&(3, 3), 0usize);
	for (x, y) in state.edge {
		counts[(
			((x / grid.width()) as isize - (repeats / 2) as isize).abs(),
			((y / grid.width()) as isize - (repeats / 2) as isize).abs(),
		)] += 1;
	}

	let block_steps = steps / grid.width();

	let rings = [
		counts[[0, 0]], // 1
		counts[[0, 1]] / 2, // 1
		counts[[1, 1]], // 4
		counts[[2, 1]], // 4
		counts[[2, 0]], // 2
	];

	rings[4] * 2
		+ (rings[3] + rings[1] * block_steps) * block_steps
		+ (rings[2] + rings[0] * (block_steps - 1)) * (block_steps - 1)
}