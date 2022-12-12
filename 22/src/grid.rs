use std::usize;

#[derive(Clone)]
pub struct Grid<T> {
	map: Vec<T>,
	width: usize,
	height: usize,
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

pub trait Size {
	fn width(&self) -> usize;
	fn height(&self) -> usize;

	fn tuple(&self) -> (usize, usize) {
		(self.width(), self.height())
	}
}

impl Size for (usize, usize) {
	fn width(&self) -> usize {self.0}
	fn height(&self) -> usize {self.1}
}

pub fn points<S: Size>(s: &S)
-> impl Iterator<Item = (usize, usize)> {
	let (width, height) = s.tuple();
	(0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))
}

pub fn adjacent4_points<S: Size>(s: &S, x: usize, y: usize)
 -> impl Iterator<Item = (usize, usize)> {
	adjacent_n_points(&ADJ4, s, x, y)
}

pub fn adjacent8_points<S: Size>(s: &S, x: usize, y: usize)
 -> impl Iterator<Item = (usize, usize)> {
	adjacent_n_points(&ADJ8, s, x, y)
}

#[inline]
fn adjacent_n_points<S: Size>(adj: &'static [(isize, isize)], s: &S, x: usize, y: usize)
 -> impl Iterator<Item = (usize, usize)> {
	let (w, h) = s.tuple();
	assert!(x < w);
	assert!(y < h);
	adj
		.iter()
		.filter_map(move |&(dx, dy)| {
			let ix = x.wrapping_add(dx as usize);
			let iy = y.wrapping_add(dy as usize);
			if ix >= w || iy >= h {
				None
			} else {
				Some((ix, iy))
			}
		})
}


impl<T: Copy> Grid<T> {
	pub fn blank<S: Size>(size: &S, v: T) -> Self {
		let (width, height) = size.tuple();
		let map = vec![v; width * height];
		Grid { map, width, height }
	}

	pub fn fill(&mut self, v: T) {
		self.map.fill(v);
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

	pub fn from_generator<S: Size, F: FnMut(usize, usize) -> T>(size: &S, mut f: F) -> Self {
		let map = points(size)
			.map(|(x, y)| f(x, y))
			.collect::<Vec<_>>();
		Grid {
			map,
			width: size.width(),
			height: size.height(),
		}
	}

	pub fn map<F: FnMut(usize, usize, &T) -> M, M>(&self, mut f: F) -> Grid<M> {
		Grid::from_generator(self, move |x, y| f(x, y, &self[[x, y]]))
	}

	pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
		&self.map[x + (y * self.width)]
	}

	pub fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
		&mut self.map[x + (y * self.width)]
	}

	pub fn get(&self, x: isize, y: isize) -> Option<&T> {
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
		adjacent8_points(self, x, y)
			.map(move |p| (p.0, p.1, &self[p]))
	}

	pub fn adjacent4<'a>(
		&'a self,
		x: usize,
		y: usize,
	) -> impl Iterator<Item = (usize, usize, &T)> + Captures<'a> {
		adjacent4_points(self, x, y)
			.map(move |p| (p.0, p.1, &self[p]))
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
				match self.get(x, y) {
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

	pub fn get_wrapped(&self, x: isize, y: isize) -> &T {
		&self.map[x.rem_euclid(self.width as isize) as usize + (y.rem_euclid(self.height as isize) as usize * self.width)]
	}
	pub fn get_wrapped_mut(&mut self, x: isize, y: isize) -> &mut T {
		&mut self.map[x.rem_euclid(self.width as isize) as usize + (y.rem_euclid(self.height as isize) as usize * self.width)]
	}

	pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
		GridIter {
			g: self,
			x: 0,
			y: 0,
		}
	}

	pub fn values_iter<'a>(&'a self) -> impl Iterator<Item = &T> + Captures<'a> {
		self.map.iter()
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

	pub fn directions_from<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item = impl Iterator<Item = (usize, usize, &'a T)>> {
		let point = [x, y];
		[
			GridRayIter{g: self, point, end: 0, step: -1, axis: 0},
			GridRayIter{g: self, point, end: self.width - 1, step: 1, axis: 0},
			GridRayIter{g: self, point, end: 0, step: -1, axis: 1},
			GridRayIter{g: self, point, end: self.height - 1, step: 1, axis: 1},
		].into_iter()
	}
}

impl<T> Size for Grid<T> {
	fn width(&self) -> usize {
		self.width
	}

	fn height(&self) -> usize {
		self.height
	}
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
	type Output = T;
	fn index(&self, index: (usize, usize)) -> &Self::Output {
		self.get_unchecked(index.0, index.1)
	}
}
impl<T> std::ops::Index<[usize; 2]> for Grid<T> {
	type Output = T;
	fn index(&self, index: [usize; 2]) -> &Self::Output {
		self.get_unchecked(index[0], index[1])
	}
}
impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
		self.get_unchecked_mut(index.0, index.1)
	}
}
impl<T> std::ops::IndexMut<[usize; 2]> for Grid<T> {
	fn index_mut(&mut self, index: [usize; 2]) -> &mut T {
		self.get_unchecked_mut(index[0], index[1])
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
impl Grid<bool> {
	pub fn print_b(&self) {
		for y in 0..self.height {
			let start = y * self.width;
			for c in &self.map[start..start + self.width] {
				print!("{}", if *c {"#"} else {"."});
			}
			println!("");
		}
	}
}
impl Grid<u8> {
	pub fn print_b(&self) {
		for y in 0..self.height {
			let start = y * self.width;
			for c in &self.map[start..start + self.width] {
				if *c > 9 {
					print!("+");
				} else {
					print!("{}", c);
				}
			}
			println!("");
		}
	}
	pub fn print_c(&self) {
		for y in 0..self.height {
			let start = y * self.width;
			for c in &self.map[start..start + self.width] {
				let c = std::char::from_u32(*c as u32).unwrap();
				print!("{}", c);
			}
			println!("");
		}
	}
}

impl <T> Grid<T>
	where T: std::cmp::PartialEq {
	pub fn find(&self, needle: &T) -> Option<(usize, usize)> {
		self.iter()
			.find(|&(_, _, v)| v == needle)
			.map(|(x, y, _)| (x, y))
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
		let v = (self.x, self.y, &self.g[[self.x, self.y]]);
		self.x += 1;
		Some(v)
	}
}

pub struct GridRayIter<'a, T> {
	g: &'a Grid<T>,
	point: [usize; 2],
	end: usize,
	step: i8,
	axis: u8,
}

impl<'a, T> Iterator for GridRayIter<'a, T> {
	type Item = (usize, usize, &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		let c = &mut self.point[self.axis as usize];
		if *c == self.end {
			return None
		}
		*c = c.wrapping_add_signed(self.step as isize);
		drop(c);

		Some((self.point[0], self.point[1], &self.g[self.point]))
	}
}