#[derive(Clone)]
pub struct Grid {
	pub map: Vec<u8>,
	pub width: usize,
	pub height: usize,
}

impl std::fmt::Debug for Grid {
	fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		Ok(())
	}
}

pub const ADJ8: [(isize, isize); 8] = [
	(-1, -1),
	(0, -1),
	(1, -1),
	(-1, 0),
	(1, 0),
	(-1, 1),
	(0, 1),
	(1, 1),
];

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

impl Grid {
	pub fn blank(width: usize, height: usize) -> Grid {
		let mut map = Vec::new();
		map.resize_with(width * height, || ' ' as u8);
		Grid { map, width, height }
	}

	pub fn new(input: &str) -> Grid {
		let input = input.as_bytes();
		let width = input
			.iter()
			.position(|&i| i == '\n' as u8)
			.expect("no newlines in input");
		// +1 for newlines
		let height = (input.len() + 1) / (width + 1);
		let map: Vec<u8> = input
			.into_iter()
			.cloned()
			.filter(|&i| i != '\n' as u8)
			.collect();
		let size = height * width;
		if map.len() != size {
			panic!("bad input: {} != {}", map.len(), size);
		}
		Grid { map, width, height }
	}

	pub fn get(&self, x: usize, y: usize) -> u8 {
		self.map[x + (y * self.width)]
	}

	pub fn set(&mut self, x: usize, y: usize, val: u8) {
		self.map[x + (y * self.width)] = val;
	}

	pub fn get_safe(&self, x: isize, y: isize) -> Option<u8> {
		if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
			None
		} else {
			Some(self.map[x as usize + (y as usize * self.width)])
		}
	}

	pub fn adjacent8<'a>(
		&'a self,
		x: usize,
		y: usize,
	) -> impl Iterator<Item = (usize, usize, u8)> + Captures<'a> {
		let (ix, iy) = (x as isize, y as isize);
		ADJ8
			.iter()
			.filter_map(move |(dx, dy)| match self.get_safe(ix + dx, iy + dy) {
				None => None,
				Some(v) => Some(((ix + dx) as usize, (iy + dy) as usize, v)),
			})
	}

	pub fn line_of_sight8<'a>(
		&'a self,
		x: usize,
		y: usize,
		test: impl Fn(u8) -> bool,
	) -> impl Iterator<Item = (usize, usize, u8)> + Captures<'a> {
		let (ix, iy) = (x as isize, y as isize);
		ADJ8.iter().filter_map(move |(dx, dy)| {
			for l in 1.. {
				let (x, y) = (ix + (dx * l), iy + (dy * l));
				match self.get_safe(x, y) {
					Some(v) if !test(v) => continue,
					Some(v) => return Some((x as usize, y as usize, v)),
					None => return None,
				}
			}
			panic!();
		})
	}

	pub fn get_wrapped_x(&self, x: usize, y: usize) -> u8 {
		self.map[(x % self.width) + (y * self.width)]
	}

	pub fn iter<'a>(&'a self) -> GridIter<'a> {
		GridIter {
			g: self,
			x: 0,
			y: 0,
		}
	}

	pub fn print(&self) {
		for y in 0..self.height {
			let start = y * self.width;
			println!("{}", std::str::from_utf8(&self.map[start..start + self.width]).unwrap());
		}
	}

	pub fn flipleftright(&self) -> Grid {
		let mut out = Grid::blank(self.width, self.height);
		for y in 0..self.height {
			for x in 0..self.width {
				out.set(self.width - x - 1, y, self.get(x, y));
			}
		}
		out
	}

	pub fn rotate(&self) -> Grid {
		let mut out = Grid::blank(self.height, self.width);
		for y in 0..self.height {
			for x in 0..self.width {
				out.set(y, self.width - x - 1, self.get(x, y));
			}
		}
		out
	}
}

pub struct GridIter<'a> {
	g: &'a Grid,
	x: usize,
	y: usize,
}

impl<'a> Iterator for GridIter<'a> {
	type Item = (usize, usize, u8);

	fn next(&mut self) -> Option<(usize, usize, u8)> {
		if self.x >= self.g.width {
			self.x = 0;
			self.y += 1;
		}
		if self.y >= self.g.height {
			return None;
		}
		let v = (self.x, self.y, self.g.get(self.x, self.y));
		self.x += 1;
		Some(v)
	}
}
