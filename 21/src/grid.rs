#[derive(Clone)]
pub struct Grid<T> {
	pub map: Vec<T>,
	pub width: usize,
	pub height: usize,
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

pub const ADJ4: [(isize, isize); 4] = [
	(0, -1),
	(-1, 0),
	(1, 0),
	(0, 1),
];

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

impl<T: Copy> Grid<T> {
	pub fn blank(width: usize, height: usize, v: T) -> Self {
		let mut map = Vec::new();
		map.resize_with(width * height, || v);
		Grid { map, width, height }
	}
}
/*
impl <T: Copy + Default> Grid<T> {
	pub fn flipleftright(&self) -> Self {
		let mut out = Grid::blank(self.width, self.height, Default::default());
		for y in 0..self.height {
			for x in 0..self.width {
				out.set(self.width - x - 1, y, self.get(x, y));
			}
		}
		out
	}

	pub fn rotate(&self) -> Self {
		let mut out = Grid::blank(self.height, self.width, Default::default());
		for y in 0..self.height {
			for x in 0..self.width {
				out.set(y, self.width - x - 1, self.get(x, y));
			}
		}
		out
	}
}*/

impl Grid<u8> {
	pub fn from_number_grid(input: &str) -> Self {
		Self::from_str_with_mapper(input, |x| *x - '0' as u8)
	}
}

impl<T> Grid<T> {
	pub fn from_str_with_mapper<F: FnMut(&u8) -> T>(input: &str, f: F) -> Self {
		let input = input.as_bytes();
		let width = input
			.iter()
			.position(|&i| i == '\n' as u8)
			.expect("no newlines in input");
		// +1 for newlines
		let height = (input.len() + 1) / (width + 1);
		let map: Vec<T> = input
			.into_iter()
			.filter(|&i| *i != '\n' as u8)
			.map(f)
			.collect();
		let size = height * width;
		if map.len() != size {
			panic!("bad input: {} != {}", map.len(), size);
		}
		Grid { map, width, height }
	}

	pub fn get(&self, x: usize, y: usize) -> &T {
		&self.map[x + (y * self.width)]
	}

	pub fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
		&mut self.map[x + (y * self.width)]
	}

	pub fn set(&mut self, x: usize, y: usize, val: T) {
		self.map[x + (y * self.width)] = val;
	}

	pub fn get_safe(&self, x: isize, y: isize) -> Option<&T> {
		if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
			None
		} else {
			Some(&self.map[x as usize + (y as usize * self.width)])
		}
	}

	pub fn adjacent8<'a>(
		&'a self,
		x: usize,
		y: usize,
	) -> impl Iterator<Item = (usize, usize, &T)> + Captures<'a> {
		let (ix, iy) = (x as isize, y as isize);
		ADJ8
			.iter()
			.filter_map(move |(dx, dy)| match self.get_safe(ix + dx, iy + dy) {
				None => None,
				Some(v) => Some(((ix + dx) as usize, (iy + dy) as usize, v)),
			})
	}

	pub fn adjacent4<'a>(
		&'a self,
		x: usize,
		y: usize,
	) -> impl Iterator<Item = (usize, usize, &T)> + Captures<'a> {
		let (ix, iy) = (x as isize, y as isize);
		ADJ4
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
		test: impl Fn(&T) -> bool,
	) -> impl Iterator<Item = (usize, usize, &T)> + Captures<'a> {
		let (ix, iy) = (x as isize, y as isize);
		ADJ8.iter().filter_map(move |(dx, dy)| {
			for l in 1.. {
				let (x, y) = (ix + (dx * l), iy + (dy * l));
				match self.get_safe(x, y) {
					Some(v) if !test(&v) => continue,
					Some(v) => return Some((x as usize, y as usize, v)),
					None => return None,
				}
			}
			panic!();
		})
	}

	pub fn get_wrapped_x(&self, x: usize, y: usize) -> &T {
		&self.map[(x % self.width) + (y * self.width)]
	}

	pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
		GridIter {
			g: self,
			x: 0,
			y: 0,
		}
	}
}

impl<T> Grid<T>
	where [T]: std::fmt::Debug {
	pub fn print(&self) {
		for y in 0..self.height {
			let start = y * self.width;
			println!("{:?}", &self.map[start..start + self.width]);
		}
	}
}

pub struct GridIter<'a, T> {
	g: &'a Grid<T>,
	x: usize,
	y: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
	type Item = (usize, usize, &'a T);

	fn next(&mut self) -> Option<(usize, usize, &'a T)> {
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
