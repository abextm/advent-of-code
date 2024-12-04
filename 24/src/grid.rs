use std::hash::Hasher;
use std::usize;
use std::ops::{Deref, DerefMut};
use memchr;

pub trait GridBacking {
	type Item;
}

impl<A: Deref<Target = [T]>, T> GridBacking for A {
	type Item = T;
}

#[derive(Clone)]
pub struct Grid<A: GridBacking> {
	map: A,
	offset: usize,
	stride: usize,
	h_stride: usize,
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


impl<T: Copy> Grid<Vec<T>> {
	pub fn blank<S: Size>(size: &S, v: T) -> Self {
		let (width, height) = size.tuple();
		let map = vec![v; width * height];
		Grid { map, width, height, offset: 0, stride: width, h_stride: 1, }
	}
}

impl<A: DerefMut<Target = [T]>, T: Copy> Grid<A> {
	pub fn fill(&mut self, v: T) {
		self.map.fill(v);
	}
}

impl<A: Copy + GridBacking> Copy for Grid<A> {
}

impl Grid<Vec<u8>> {
	pub fn from_number_grid(input: &str) -> Self {
		Self::from_str_with_mapper(input, |x| *x - b'0')
	}
}

impl<'a> Grid<&'a [u8]> {
	pub fn from_char_grid<T: AsRef<[u8]> + ?Sized>(input: &'a T) -> Self {
		let input = input.as_ref();
		let width = memchr::memchr(b'\n', input)
			.unwrap_or(input.len());
		let stride = width + 1;
		Grid {
			map: input,
			offset: 0,
			height: (input.len() + 1) / stride,
			h_stride: 1,
			width,
			stride,
		}
	}

	pub fn from_char_grid_list(input: &'a str) -> GridListIter<'a> {
		GridListIter { s: input.as_bytes() }
	}
}

impl<A: Deref<Target = [T]>, T: Clone> Grid<A> {
	pub fn owned_copy(&self) -> Grid<Vec<T>> {
		Grid {
			map: self.map.to_vec(),
			offset: self.offset,
			stride: self.stride,
			h_stride: self.h_stride,
			width: self.width,
			height: self.height,
		}
	}
}

impl<T> Grid<Vec<T>> {
	pub fn from_str_with_mapper<F: FnMut(&u8) -> T>(input: &str, f: F) -> Self {
		let input = input.as_bytes();
		let width = memchr::memchr(b'\n', input)
			.unwrap_or(input.len());
		// +1 for newlines
		let height = (input.len() + 1) / (width + 1);
		let map: Vec<T> = input
			.into_iter()
			.filter(|&i| *i != b'\n')
			.map(f)
			.collect();
		let size = height * width;
		if map.len() != size {
			panic!("bad input: {} != {}", map.len(), size);
		}
		Grid { map, width, height, offset: 0, stride: width, h_stride: 1 }
	}

	pub fn from_generator<S: Size, F: FnMut(usize, usize) -> T>(size: &S, mut f: F) -> Self {
		let map = points(size)
			.map(|(x, y)| f(x, y))
			.collect::<Vec<_>>();
		Grid {
			map,
			width: size.width(),
			stride: size.width(),
			height: size.height(),
			h_stride: 1,
			offset: 0,
		}
	}
}

impl<'a, A: Deref<Target = [T]>, T: 'a> Grid<A> {
	#[inline]
	fn index(&self, x: usize, y: usize) -> usize {
		self.offset + x * self.h_stride + y * self.stride
	}

	pub fn map<F: FnMut(usize, usize, &T) -> M, M>(&self, mut f: F) -> Grid<Vec<M>> {
		Grid::from_generator(self, move |x, y| f(x, y, &self[[x, y]]))
	}

	pub fn transposed(self) -> Self {
		Grid {
			map: self.map,
			offset: self.offset,
			stride: self.h_stride,
			h_stride: self.stride,
			width: self.height,
			height: self.width,
		}
	}

	pub fn as_ref(&self) -> Grid<&[T]> {
		Grid {
			map: &self.map,
			offset: self.offset,
			stride: self.stride,
			h_stride: self.h_stride,
			width: self.width,
			height: self.height,
		}
	}

	pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
		&self.map[self.index(x, y)]
	}

	pub fn get(&self, x: isize, y: isize) -> Option<&T> {
		if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
			None
		} else {
			Some(&self.map[self.index(x as usize, y as usize)])
		}
	}

	pub fn adjacent8(
		&'a self,
		x: usize,
		y: usize,
	) -> impl Iterator<Item = (usize, usize, &T)> + Captures<'a> {
		adjacent8_points(self, x, y)
			.map(move |p| (p.0, p.1, &self[p]))
	}

	pub fn adjacent4(
		&'a self,
		x: usize,
		y: usize,
	) -> impl Iterator<Item = (usize, usize, &T)> + Captures<'a> {
		adjacent4_points(self, x, y)
			.map(move |p| (p.0, p.1, &self[p]))
	}

	pub fn line_of_sight8(
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

	// may be faster when finding a small number of elements than .iter().filter
	pub fn filter_enumerate<P: FnMut(&T) -> bool>(&'a self, mut predicate: P) -> impl Iterator<Item = (usize, usize, &T)> + Captures<'a> {
		let stride = self.stride.max(self.h_stride);
		let swap = self.h_stride > self.stride;
		self.map[self.index(0, 0)..=self.index(self.width() - 1, self.height() - 1)].iter()
			.enumerate()
			.filter(move |(_, v)| predicate(v))
			.map(move |(i, v)| {
				let mut  coord = (i % stride, i / stride);
				if swap {
					coord = (coord.1, coord.0);
				}
				(coord.0, coord.1, v)
			})
			.filter(|&(x, _y, _v)| x < self.width)
	}

	pub fn get_wrapped(&self, x: isize, y: isize) -> &T {
		&self.map[self.index(x.rem_euclid(self.width as isize) as usize, y.rem_euclid(self.height as isize) as usize)]
	}

	pub fn iter(&'a self) -> GridIter<'a, A, T> {
		GridIter {
			g: self,
			x: 0,
			y: 0,
		}
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

	pub fn directions_from(&'a self, x: usize, y: usize) -> impl Iterator<Item = impl Iterator<Item = (usize, usize, &'a T)>> {
		let point = [x, y];
		[
			GridRayIter{g: self, point, end: 0, step: -1, axis: 0},
			GridRayIter{g: self, point, end: self.width - 1, step: 1, axis: 0},
			GridRayIter{g: self, point, end: 0, step: -1, axis: 1},
			GridRayIter{g: self, point, end: self.height - 1, step: 1, axis: 1},
		].into_iter()
	}
}

impl<A: DerefMut<Target=[T]>, T> Grid<A> {
	pub fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
		let index = self.index(x, y);
		&mut self.map[index]
	}
	pub fn get_wrapped_mut(&mut self, x: isize, y: isize) -> &mut T {
		let index = self.index(x.rem_euclid(self.width as isize) as usize, y.rem_euclid(self.height as isize) as usize);
		&mut self.map[index]
	}

	pub fn as_mut_ref(&mut self) -> Grid<&mut [T]> {
		Grid {
			map: &mut self.map,
			offset: self.offset,
			stride: self.stride,
			h_stride: self.h_stride,
			width: self.width,
			height: self.height,
		}
	}
}

impl<A: Deref<Target = [T]>, T> Size for Grid<A> {
	fn width(&self) -> usize {
		self.width
	}

	fn height(&self) -> usize {
		self.height
	}
}

trait GridIndex {
	fn index(self) -> (usize, usize);
}
impl<T: GridIndexInt> GridIndex for [T; 2] {
	fn index(self) -> (usize, usize) {
		(self[0].as_usize(), self[1].as_usize())
	}
}
impl<T: GridIndexInt> GridIndex for (T, T) {
	fn index(self) -> (usize, usize) {
		(self.0.as_usize(), self.1.as_usize())
	}
}
trait GridIndexInt: Copy {
	fn as_usize(self) -> usize;
}
impl GridIndexInt for usize { fn as_usize(self) -> usize { self } }
impl GridIndexInt for isize { fn as_usize(self) -> usize { self as usize } }
impl GridIndexInt for i64 { fn as_usize(self) -> usize { self as usize } }
impl GridIndexInt for u64 { fn as_usize(self) -> usize { self as usize } }
impl GridIndexInt for i32 { fn as_usize(self) -> usize { self as usize } }
impl GridIndexInt for u32 { fn as_usize(self) -> usize { self as usize } }
impl GridIndexInt for i16 { fn as_usize(self) -> usize { self as usize } }
impl GridIndexInt for u16 { fn as_usize(self) -> usize { self as usize } }
impl GridIndexInt for i8 { fn as_usize(self) -> usize { self as usize } }
impl GridIndexInt for u8 { fn as_usize(self) -> usize { self as usize } }

impl<A: Deref<Target = [T]>, T, I: GridIndex> std::ops::Index<I> for Grid<A> {
	type Output = T;
	fn index(&self, index: I) -> &Self::Output {
		let index = index.index();
		self.get_unchecked(index.0, index.1)
	}
}
impl<A: DerefMut<Target = [T]>, T, I: GridIndex> std::ops::IndexMut<I> for Grid<A> {
	fn index_mut(&mut self, index: I) -> &mut T {
		let index = index.index();
		self.get_unchecked_mut(index.0, index.1)
	}
}

impl<A: Deref<Target = [T]>, T: std::fmt::Debug> Grid<A> {
	pub fn print(&self) {
		for y in 0..self.height {
			for x in 0..self.width {
				print!("{:?}\t", &self[[x, y]]);
			}
			println!("");
		}
	}
}

impl<A: Deref<Target = [T]>, T> Grid<A> {
	pub fn print_mapped<F: Fn(&T) -> char>(&self, convert: F) {
		for y in 0..self.height {
			for x in 0..self.width {
				print!("{}", convert(&self[[x, y]]));
			}
			println!("");
		}
	}
}
impl<A: Deref<Target = [bool]>> Grid<A> {
	pub fn print_bool(&self) {
		self.print_mapped(|&c| if c {'#'} else {'.'});
	}
}
impl<A: Deref<Target = [u8]>> Grid<A> {
	pub fn print_b(&self) {
		self.print_mapped(|&v| if v > 9 { '+' } else { (v + b'0') as char });
	}
	pub fn print_c(&self) {
		self.print_mapped(|&v| v as char);
	}
}

impl<A: Deref<Target = [T]>, T> Grid<A>
	where T: std::cmp::PartialEq {
	pub fn find(&self, needle: &T) -> Option<(usize, usize)> {
		self.iter()
			.find(|&(_, _, v)| v == needle)
			.map(|(x, y, _)| (x, y))
	}
}

pub struct GridIter<'a, A: Deref<Target = [T]>, T> {
	g: &'a Grid<A>,
	x: usize,
	y: usize,
}

impl<'a, A: Deref<Target = [T]>, T: 'a> Iterator for GridIter<'a, A, T> {
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

pub struct GridRayIter<'a, A: Deref<Target = [T]>, T> {
	g: &'a Grid<A>,
	point: [usize; 2],
	end: usize,
	step: i8,
	axis: u8,
}

impl<'a, A: Deref<Target = [T]>, T: 'a> Iterator for GridRayIter<'a, A, T> {
	type Item = (usize, usize, &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		let c = &mut self.point[self.axis as usize];
		if *c == self.end {
			return None
		}
		*c = c.wrapping_add_signed(self.step as isize);

		Some((self.point[0], self.point[1], &self.g[self.point]))
	}
}

pub struct GridListIter<'a> {
	s: &'a [u8],
}

impl<'a> Iterator for GridListIter<'a> {
	type Item = Grid<&'a [u8]>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.s.len() < 2 {
			return None
		}

		let width = memchr::memchr(b'\n', self.s)
			.unwrap_or(self.s.len());

		let stride = width + 1;
		let mut off = width + 1;

		loop {
			off += stride;
			if off >= self.s.len() || self.s[off] == b'\n' {
				break;
			}
		}

		let chunk = &self.s[..(self.s.len().min(off - 1))];
		self.s = &self.s[(self.s.len().min(off + 1))..];

		Some(Grid {
			map: chunk,
			offset: 0,
			height: (chunk.len() + 1) / stride,
			h_stride: 1,
			width,
			stride,
		})
	}
}

impl<A: Deref<Target=[T]>, A2: Deref<Target=[T2]>, T: PartialEq<T2>, T2> PartialEq<Grid<A2>> for Grid<A> {
	fn eq(&self, other: &Grid<A2>) -> bool {
		if self.width() != other.width() || self.height() != other.height() {
			return false;
		}

		return self.iter().all(|(x, y, v)| *v == other[[x, y]])
	}
}

impl<A: Deref<Target=[T]>, T: Eq> Eq for Grid<A> {
}

impl<A: Deref<Target=[T]>, T: std::hash::Hash> std::hash::Hash for Grid<A> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.iter().for_each(|(_x, _y, v)| v.hash(state));
	}
}

/**
 * incorrect for unequally transposed or strided grids
 */
#[derive(Eq, PartialEq)]
pub struct FastHash<const ROWS: usize, T: Clone + std::hash::Hash>(pub Grid<Vec<T>>);

impl<const ROWS: usize, T: Clone + std::hash::Hash> std::hash::Hash for FastHash<ROWS, T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let g = &self.0;
		if g.height < ROWS * 2 {
			T::hash_slice(&g.map[g.index(0, 0)..=g.index(g.width() - 1, g.height() - 1)], state)
		} else {
			T::hash_slice(&g.map[g.index(0, 0)..=g.index(g.width() - 1, ROWS)], state);
			T::hash_slice(&g.map[g.index(0, g.height() - 1 - ROWS)..=g.index(g.width() - 1, g.height() - 1)], state);
		}
	}
}

