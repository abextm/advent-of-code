use std::mem;

pub trait TakeN<T> {
	fn take_n<const N: usize>(&mut self) -> [T; N];
}

impl<I, T> TakeN<T> for I
where
	I: Iterator<Item = T>,
{
	fn take_n<const N: usize>(&mut self) -> [T; N] {
		unsafe {
			let mut out: mem::MaybeUninit<[T; N]> = mem::MaybeUninit::uninit();
			{
				let uninitout: &mut [mem::MaybeUninit<T>; N] = mem::transmute(&mut out);
				for elem in &mut uninitout[..] {
					*elem = mem::MaybeUninit::new(self.next().expect("not enough items in iterator"));
				}
			}
			out.assume_init()
		}
	}
}

pub trait TakeNDeref<T> {
	fn take_n_deref<const N: usize>(&mut self) -> [T; N];
}

impl<I, T, R> TakeNDeref<T> for I
where
	I: Iterator<Item = R>,
	R: std::ops::Deref<Target = T>,
	T: Copy,
{
	fn take_n_deref<const N: usize>(&mut self) -> [T; N] {
		unsafe {
			let mut out: mem::MaybeUninit<[T; N]> = mem::MaybeUninit::uninit();
			{
				let uninitout: &mut [mem::MaybeUninit<T>; N] = mem::transmute(&mut out);
				for elem in &mut uninitout[..] {
					*elem = mem::MaybeUninit::new(*self.next().expect("not enough items in iterator"));
				}
			}
			out.assume_init()
		}
	}
}

pub fn generate<T: Copy, F: FnMut(T, usize) -> T, const N: usize>(init: T, mut gen: F) -> [T; N] {
	unsafe {
		let mut out: mem::MaybeUninit<[T; N]> = mem::MaybeUninit::uninit();
		{
			let mut last = init;
			let uninitout: &mut [mem::MaybeUninit<T>; N] = mem::transmute(&mut out);
			for i in 0..N {
				last = gen(last, i);
				uninitout[i] = mem::MaybeUninit::new(last);
			}
		}
		out.assume_init()
	}
}

#[test]
fn test() {
	let mut iter = [1, 2].iter();
	let [a, b]: [&i32; 2] = iter.take_n();
	assert_eq!(*a, 1);
	assert_eq!(*b, 2);
}
#[test]
fn test_deref() {
	let mut iter = [1, 2i32].iter();
	let [a, b]: [i32; 2] = iter.take_n_deref();
	assert_eq!(a, 1);
	assert_eq!(b, 2);
}
