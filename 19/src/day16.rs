#[aoc(part1 = 70856418)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let mut v = input.trim().bytes().map(|x| x - b'0').collect::<Vec<_>>();

	for _it in 0..100 {
		for i in 0..v.len() {
			let mut j = i;
			let step = i + 1;
			let mut sum: isize = 0;
			while j < v.len() {
				for jj in j..(j + step).min(v.len()) {
					sum += v[jj] as isize;
				}
				j += step * 2;
				for jj in j..(j + step).min(v.len()) {
					sum -= v[jj] as isize;
				}
				j += step * 2;
			}

			v[i] = (sum.abs() % 10) as u8;
		}
	}

	v[..8].iter().fold(0, |a, b| a * 10 + *b as usize)
}

#[aoc(part2=87766336)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let position: usize = input[..7].parse().unwrap();
	let input = input.trim().bytes().map(|x| x - b'0').collect::<Vec<_>>();

	let size = input.len() * 10_000 - position;
	let mut v = Vec::with_capacity(size);
	{
		let pad = position % input.len();
		v.extend_from_slice(&input[pad..]);

		for i in 0..(size / input.len()) {
			v.extend_from_slice(&input);
		}

		assert_eq!(v.len(), size);
	}

	for _it in 0..100 {
		for i in (1..v.len()).rev() {
			v[i - 1] = (v[i - 1] + v[i]) % 10;
		}
	}

	v[..8].iter().fold(0, |a, b| a * 10 + *b as usize)
}