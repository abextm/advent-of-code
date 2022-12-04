pub trait TakeN<T> {
	fn take_n<const N: usize>(&mut self) -> Option<[T; N]>;
}

impl<I, T> TakeN<T> for I
where
	I: Iterator<Item = T>,
{
	fn take_n<const N: usize>(&mut self) -> Option<[T; N]> {
		self.take(N).collect::<Vec<_>>().try_into().ok()
	}
}

use std::str::FromStr;

pub trait IterFromStr<S: AsRef<str>, I: Iterator<Item = S>> {
	fn must_parse<T: FromStr>(&mut self) -> core::iter::Map<&mut I, fn(S) -> T>;
}

fn must_parse<S: AsRef<str>, T: FromStr>(s: S) -> T {
	match s.as_ref().parse::<T>() {
		Ok(v) => v,
		Err(_) => panic!("error parsing \"{}\"", s.as_ref())
	}
}

impl<S: AsRef<str>, I: Iterator<Item = S>> IterFromStr<S, I> for I {
	fn must_parse<T: FromStr>(&mut self) -> core::iter::Map<&mut I, fn(S) -> T> {
		self.map(must_parse::<S, T>)
	}
}