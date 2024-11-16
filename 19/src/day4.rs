use crate::taken::TakeN;

#[aoc(part1=1767, part2=1192)]
fn day4(input: &str, part2: bool) -> usize {
	let [lower_bound, upper_bound]: [u32; 2] = input.trim().split('-').map(|s| s.parse().unwrap()).take_n().unwrap();
	(lower_bound..upper_bound)
		.filter(|n| test(*n, part2))
		.count()
}

fn zip_pair<I, V>(iter: I) -> impl Iterator<Item = (V, V)>
where
	I: IntoIterator<Item = V> + Clone,
{
	iter.clone().into_iter().zip(iter.into_iter().skip(1))
}

fn test(n: u32, part2: bool) -> bool {
	let mut l: [u32; 6] = [0; 6];
	let mut decimal = 1;
	for val in l.iter_mut() {
		*val = (n / decimal) % 10;
		decimal *= 10;
	}

	!zip_pair(&l.clone()).any(|v| v.0 < v.1)
		&& if !part2 {
			zip_pair(&l).any(|t| *t.0 == *t.1)
		} else {
			let mut had_double = false;
			let mut last = l[0];
			let mut num_last = 0;
			for i in 1..l.len() {
				if l[i] == last {
					num_last += 1;
				} else {
					if num_last == 1 {
						had_double = true;
					}
					num_last = 0;
				}
				last = l[i];
			}
			had_double || num_last == 1
		}
}

#[test]
fn test_ok() {
	assert_eq!(test(111111, false), true);
}

#[test]
fn test_decrease() {
	assert_eq!(test(223450, false), false);
}

#[test]
fn test_nodouble() {
	assert_eq!(test(123789, false), false);
}

#[test]
fn test_ok2() {
	assert_eq!(test(112233, true), true);
}

#[test]
fn test_ok3() {
	assert_eq!(test(111122, true), true);
}

#[test]
fn test_nodouble_butmore() {
	assert_eq!(test(123444, true), false);
}
