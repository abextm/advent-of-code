#[aoc(day15, part1)]
fn day15_part1(input: &str) -> i64 {
	run(input, 2020)
}

#[aoc(day15, part2)]
fn day15_part2(input: &str) -> i64 {
	run(input, 30_000_000)
}

struct HashBuilder {}
struct Hasher {
	val: u64,
}
impl std::hash::BuildHasher for HashBuilder {
	type Hasher = Hasher;

	#[inline(always)]
	fn build_hasher(&self) -> Hasher {
		Hasher{
			val: 0,
		}
	}
}
impl std::hash::Hasher for Hasher {
	#[inline(always)]
	fn finish(&self) -> u64 {
		self.val
	}

	#[inline(always)]
	fn write(&mut self, bytes: &[u8]) {
		assert_eq!(bytes.len(), 8);
		//let v = u64::from_ne_bytes(bytes.try_into().unwrap());
		let v = *unsafe {std::mem::transmute::<&u8, &u64>(&bytes[0])};
		let mut x = v;
		(x, _) = (x ^ (x >> 30)).overflowing_mul(0xbf58476d1ce4e5b9);
		// reserve the first 128 entries for small values, which we hit often. keep this in cache
		x |= 128;
		self.val = if v < 128 { v } else { x }
	}
}

fn run(input: &str, end: i64) -> i64 {
	let mut map = std::collections::HashMap::<i64, i64, _>::with_capacity_and_hasher(
		0x380000,
		HashBuilder{},
	);
	let mut turn = 0;
	let mut last: (i64, i64) = (0, 0);
	for num in input.split(",").map(|v| v.parse::<i64>().unwrap()) {
		last = tick(&mut map, num, turn);
		turn += 1;
	}
	for turn in turn..end {
		last = tick(&mut map, last.1, turn);
	}

	last.0
}

#[inline(always)]
fn tick<H: std::hash::BuildHasher>(map: &mut std::collections::HashMap<i64, i64, H>, num: i64, turn: i64) -> (i64, i64) {
	let d = turn - map.insert(num, turn).unwrap_or(turn);
	(num, d)
}

#[test]
fn testp1() {
	assert_eq!(run("0,3,6", 9), 4);
	assert_eq!(run("1,3,2", 2020), 1);
	assert_eq!(run("2,1,3", 2020), 10);
}
