pub trait IterSortEx<T> {
	fn sort_by_key<K, F>(self, f: F) -> std::vec::IntoIter<T>
	where
		F: FnMut(&T) -> K,
		K: Ord;

	fn sort_by_cached_key<K, F>(self, f: F) -> std::vec::IntoIter<T>
	where
		F: FnMut(&T) -> K,
		K: Ord;

	fn sort_by<F>(&mut self, compare: F) -> std::vec::IntoIter<T>
	where
		F: FnMut(&T, &T) -> std::cmp::Ordering;
}

impl<I, T> IterSortEx<T> for I
where
	I: Iterator<Item = T>,
{
	fn sort_by_key<K, F>(self, f: F) -> std::vec::IntoIter<T>
	where
		F: FnMut(&T) -> K,
		K: Ord,
	{
		let mut vec: Vec<T> = self.collect();
		vec.sort_by_key(f);
		vec.into_iter()
	}

	fn sort_by_cached_key<K, F>(self, f: F) -> std::vec::IntoIter<T>
	where
		F: FnMut(&T) -> K,
		K: Ord,
	{
		let mut vec: Vec<T> = self.collect();
		vec.sort_by_key(f);
		vec.into_iter()
	}

	fn sort_by<F>(&mut self, compare: F) -> std::vec::IntoIter<T>
	where
		F: FnMut(&T, &T) -> std::cmp::Ordering,
	{
		let mut vec: Vec<T> = self.collect();
		vec.sort_by(compare);
		vec.into_iter()
	}
}
