#![allow(dead_code)]

use std::{array};
use std::cell::OnceCell;
use std::ops::{Add, Deref, DerefMut, Index, IndexMut, Mul, Rem, Sub, Div, Neg};
use memchr::memchr_iter;
use strength_reduce::StrengthReducedUsize;

#[derive(Clone)]
pub struct Grid<const ND: usize, M> {
	pub array: M,
	stride: Ve<ND>,
	shape: Ve<ND>,
	stride_order: [u8; ND],
	reduced_stride: OnceCell<[StrengthReducedUsize; ND]>,
}

pub fn adj4<const ND: usize>() -> impl Iterator<Item=Ve<ND>> {
	(0..(2 * ND)).map(|i| {
		let mut v = [0; ND];
		v[i >> 1] = if i & 1 == 0 { -1 } else { 1 };
		Ve(v)
	})
}

pub fn adj8<const ND: usize>() -> impl Iterator<Item=Ve<ND>> {
	(0..3isize.pow(ND as u32)).filter_map(|mut pt| {
		let ve = Ve(array::from_fn(|_| {
			let r = pt % 3;
			pt /= 3;
			r - 1
		}));
		if ve == Ve::zero() {
			None
		} else {
			Some(ve)
		}
	})
}

impl<const ND: usize, M> Grid<ND, M> {
	fn idx(&self, pt: Ve<ND>) -> Option<usize> {
		let mut sum = 0;
		for i in 0..ND {
			let a = pt[i];
			if a < 0 || a >= self.shape[i] {
				return None;
			}
			sum += a as usize * self.stride[i] as usize;
		}
		Some(sum)
	}

	fn idx_wrapped(&self, pt: Ve<ND>) -> usize {
		let mut sum = 0;
		for i in 0..ND {
			let a = pt[i].rem_euclid(self.shape[i]);
			sum += a as usize * self.stride[i] as usize;
		}
		sum
	}

	fn de_idx_fn<'a>(&'a self) -> impl Fn(usize) -> Option<Ve<ND>> + 'a {
		let stride = self.reduced_stride.get_or_init(|| {
			self.stride.0.map(|stride| StrengthReducedUsize::new(stride as usize))
		});

		move |index| {
			let mut pt = Ve::zero();

			for i in 0..ND {
				let mut d = index / stride[self.stride_order[i] as usize];
				if i < ND - 1 {
					d = d % stride[self.stride_order[i + 1] as usize];
				}
				if d >= self.shape[self.stride_order[i] as usize] as usize {
					return None;
				}
				pt[i] = d as isize;
			}

			Some(pt)
		}
	}
}

impl<const ND: usize, V: Into<Ve<ND>>, T, M: Deref<Target=[T]>> Index<V> for Grid<ND, M> {
	type Output = T;

	fn index(&self, index: V) -> &Self::Output {
		self.get(index).expect("out of bounds")
	}
}

impl<const ND: usize, V: Into<Ve<ND>>, T, M: DerefMut<Target=[T]>> IndexMut<V> for Grid<ND, M> {
	fn index_mut(&mut self, index: V) -> &mut Self::Output {
		let idx = self.idx(index.into()).expect("out of bounds");
		&mut self.array[idx]
	}
}

impl<const ND: usize, M> Grid<ND, M> {
	pub fn transpose(&mut self, indexes: [usize; ND]) {
		self.stride = Ve(array::from_fn(|i| self.stride.0[indexes[i]]));
		self.shape = Ve(array::from_fn(|i| self.shape.0[indexes[i]]));
		self.stride_order = self.stride_order.map(|v| indexes[v as usize] as u8);
		if let Some(rs) = self.reduced_stride.get_mut() {
			*rs = array::from_fn(|i| rs[indexes[i]]);
		}
	}
}

impl<const ND: usize, T, M: Deref<Target=[T]>> Grid<ND, M> {
	pub fn get<V: Into<Ve<ND>>>(&self, index: V) -> Option<&T> {
		self.idx(index.into()).map(|idx| &self.array[idx])
	}

	pub fn get_wrapped<V: Into<Ve<ND>>>(&self, index: V) -> &T {
		&self.array[self.idx_wrapped(index.into())]
	}

	pub fn filter_enumerate<'a>(&'a self, mut filter: impl FnMut(&T) -> bool + 'a) -> impl Iterator<Item=(Ve<ND>, &'a T)> + 'a
	where
		T: 'a,
	{
		let de_idx = self.de_idx_fn();
		self.array.iter().enumerate()
			.filter(move |(_i, v)| filter(v))
			.filter_map(move |(i, v)| de_idx(i).map(|i| (i, v)))
	}

	pub fn as_ref(&self) -> Grid<ND, &[T]> {
		Grid {
			array: &self.array,
			stride: self.stride,
			shape: self.shape,
			stride_order: self.stride_order,
			reduced_stride: self.reduced_stride.clone(),
		}
	}

	pub fn map<J>(&self, f: impl Fn(Ve<ND>, &T) -> J) -> Grid<ND, Vec<J>> {
		let mut stride = 1;
		let strides = Ve(self.shape.0.map(|dim| {
			let v = stride;
			stride *= dim;
			v
		}));
		let mut out = Vec::with_capacity(stride as usize);
		let stride_order = array::from_fn(|i| i as u8);
		let it = GridPointIter {
			shape: self.shape,
			stride_order,
			pos: Some([0usize; ND].into()),
		};

		for pt in it {
			out.push(f(pt, &self[pt]));
		}

		Grid {
			array: out,
			stride: strides,
			shape: self.shape,
			stride_order,
			reduced_stride: OnceCell::new(),
		}
	}

	pub fn from_slice(shape: impl Into<Ve<ND>>, val: M) -> Self {
		let mut stride = 1;
		let shape = shape.into();
		let strides = Ve(shape.0.map(|dim| {
			let v = stride;
			stride *= dim;
			v
		}));
		assert_eq!(val.len(), stride as usize);
		Grid {
			array: val,
			stride: strides,
			shape,
			stride_order: array::from_fn(|i| i as u8),
			reduced_stride: OnceCell::new(),
		}
	}
}

impl<const ND: usize, T: Clone, M: Deref<Target=[T]>> Grid<ND, M> {
	pub fn cloned(&self) -> Grid<ND, Vec<T>> {
		Grid {
			array: Vec::from(&self.array as &[T]),
			stride: self.stride,
			shape: self.shape,
			stride_order: self.stride_order,
			reduced_stride: self.reduced_stride.clone(),
		}
	}
}

impl<const ND: usize, T: Clone> Grid<ND, Vec<T>> {
	pub fn new(shape: impl Into<Ve<ND>>, val: T) -> Self {
		let mut stride = 1;
		let shape = shape.into();
		let strides = Ve(shape.0.map(|dim| {
			let v = stride;
			stride *= dim;
			v
		}));
		Grid {
			array: vec![val; stride as usize],
			stride: strides,
			shape,
			stride_order: array::from_fn(|i| i as u8),
			reduced_stride: OnceCell::new(),
		}
	}
}

impl<const ND: usize, M: Deref<Target=[u8]>> Grid<ND, M> {
	pub fn find<'a>(&'a self, needle: u8) -> impl Iterator<Item=Ve<ND>> + 'a {
		let de_idx = self.de_idx_fn();
		memchr_iter(needle, &self.array)
			.filter_map(de_idx)
	}
}

impl<const ND: usize, T, M: DerefMut<Target=[T]>> Grid<ND, M> {
	pub fn get_mut<V: Into<Ve<ND>>>(&mut self, index: V) -> Option<&mut T> {
		self.idx(index.into()).map(|idx| &mut self.array[idx])
	}

	pub fn get_mut_wrapped<V: Into<Ve<ND>>>(&mut self, index: V) -> &mut T {
		let idx = self.idx_wrapped(index.into());
		&mut self.array[idx]
	}
}

impl Grid<2, &[u8]> {
	pub fn from_char_grid(cg: &(impl AsRef<[u8]> + ?Sized)) -> Grid<2, &[u8]> {
		let input = cg.as_ref();
		let width = memchr::memchr(b'\n', input)
			.unwrap_or(input.len());
		let stride = width + 1;
		let height = (input.len() + 1) / stride;
		Grid {
			array: input,
			stride: [1, stride].into(),
			shape: [width, height].into(),
			stride_order: [0, 1],
			reduced_stride: OnceCell::new(),
		}
	}
}

impl Grid<3, &[u8]> {
	pub fn from_char_grid_list(cg: &(impl AsRef<[u8]> + ?Sized)) -> Grid<3, &[u8]> {
		let input = cg.as_ref();
		let width = memchr::memchr(b'\n', input)
			.unwrap_or(input.len());
		let stride = width + 1;
		let mut h_stride = 0;
		while h_stride < input.len() {
			if input[h_stride] == b'\n' {
				while input[h_stride] == b'\n' {
					h_stride += 1;
				}
				h_stride -= 1;
				break;
			}
			h_stride += stride;
		}
		let depth = (input.len() + 1) / h_stride;
		let height = h_stride / stride;
		Grid {
			array: input,
			stride: [1, stride, h_stride].into(),
			shape: [width, height, depth].into(),
			stride_order: [0, 1, 2],
			reduced_stride: OnceCell::new(),
		}
	}
}

pub struct GridPointIter<const ND: usize> {
	shape: Ve<ND>,
	stride_order: [u8; ND],
	pos: Option<Ve<ND>>,
}

impl<const ND: usize> Iterator for GridPointIter<ND> {
	type Item = Ve<ND>;

	fn next(&mut self) -> Option<Self::Item> {
		let out = self.pos;
		if let Some(mut p) = self.pos {
			self.pos = 'overflow: {
				for i in self.stride_order {
					let v = p[i as usize] + 1;
					if v >= self.shape[i as usize] {
						p[i as usize] = 0;
					} else {
						p[i as usize] = v;
						break 'overflow Some(p);
					}
				}

				None
			}
		}

		out
	}
}


impl<const ND: usize, M> Grid<ND, M> {
	pub fn shape(&self) -> Ve<ND> {
		self.shape
	}

	pub fn points(&self) -> GridPointIter<ND> {
		GridPointIter {
			shape: self.shape,
			stride_order: self.stride_order,
			pos: Some([0usize; ND].into()),
		}
	}
}
impl<const ND: usize, T, M: Deref<Target=[T]>> Grid<ND, M> {
	pub fn iter<'a>(&'a self) -> impl Iterator<Item=(Ve<ND>, &'a T)> + 'a
	where
		T: 'a,
	{
		self.points().map(|p| (p, &self[p]))
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ve<const N: usize>(pub [isize; N]);

impl<const N: usize> Ve<N> {
	pub fn zero() -> Self {
		Ve([0; N])
	}
}

impl<const N: usize> From<[isize; N]> for Ve<N> {
	fn from(value: [isize; N]) -> Self {
		Self(value)
	}
}

impl<const N: usize> From<[usize; N]> for Ve<N> {
	fn from(value: [usize; N]) -> Self {
		Self(value.map(|n| n as _))
	}
}
impl<const N: usize> From<[u64; N]> for Ve<N> {
	fn from(value: [u64; N]) -> Self {
		Self(value.map(|n| n as _))
	}
}
impl<const N: usize> From<[u32; N]> for Ve<N> {
	fn from(value: [u32; N]) -> Self {
		Self(value.map(|n| n as _))
	}
}
impl<const N: usize> From<[i32; N]> for Ve<N> {
	fn from(value: [i32; N]) -> Self {
		Self(value.map(|n| n as _))
	}
}
impl<const N: usize> From<[u16; N]> for Ve<N> {
	fn from(value: [u16; N]) -> Self {
		Self(value.map(|n| n as _))
	}
}
impl<const N: usize> From<[u8; N]> for Ve<N> {
	fn from(value: [u8; N]) -> Self {
		Self(value.map(|n| n as _))
	}
}
impl<const N: usize> From<isize> for Ve<N> {
	fn from(value: isize) -> Self {
		Self([value; N])
	}
}
impl<const N: usize> From<usize> for Ve<N> {
	fn from(value: usize) -> Self {
		Self([value as isize; N])
	}
}

impl<const N: usize> From<Ve<N>> for [isize; N] {
	fn from(value: Ve<N>) -> Self {
		value.0
	}
}

impl<const N: usize> Index<usize> for Ve<N> {
	type Output = isize;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl<const N: usize> IndexMut<usize> for Ve<N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}

macro_rules! vec_op {
  ($tr:ident, $f: ident) => {
	  impl<const N: usize> $tr for &Ve<N> {
			type Output = Ve<N>;

			fn $f(self, rhs: Self) -> Self::Output {
				Ve(array::from_fn(|i| $tr::$f(self[i], rhs[i])))
			}
		}
	  impl<const N: usize> $tr for Ve<N> {
			type Output = Ve<N>;

			fn $f(self, rhs: Self) -> Self::Output {
				Ve(array::from_fn(|i| $tr::$f(self[i], rhs[i])))
			}
		}
	  impl<const N: usize> $tr<Ve<N>> for &Ve<N> {
			type Output = Ve<N>;

			fn $f(self, rhs: Ve<N>) -> Self::Output {
				Ve(array::from_fn(|i| $tr::$f(self[i], rhs[i])))
			}
		}
	  impl<const N: usize> $tr<&Ve<N>> for Ve<N> {
			type Output = Ve<N>;

			fn $f(self, rhs: &Ve<N>) -> Self::Output {
				Ve(array::from_fn(|i| $tr::$f(self[i], rhs[i])))
			}
		}
  };
}

impl<const N: usize> Neg for Ve<N> {
	type Output = Ve<N>;

	fn neg(self) -> Self::Output {
		Ve(self.0.map(|v| -v))
	}
}

vec_op!(Add, add);
vec_op!(Sub, sub);
vec_op!(Mul, mul);
vec_op!(Div, div);
vec_op!(Rem, rem);