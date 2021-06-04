#[aoc(day25, part1)]
fn day25_part1(input: &str) -> u64 {
	let input = input
		.trim()
		.split("\n")
		.map(|x| x.parse().unwrap())
		.map(|k| (k, find_loop_size(7, k)))
		.collect::<Vec<_>>();
	let card = input[0];
	let door = input[1];
	let key = do_loop(card.0, door.1);

	key
}

fn find_loop_size(subject: u64, pkey: u64) -> u64 {
	let mut val: u64 = 1;
	for i in 1.. {
		val = (val * subject) % 20201227;
		if val == pkey {
			return i;
		}
	}
	panic!();
}

fn do_loop(subject: u64, iterations: u64) -> u64 {
	let mut key: u64 = 1;
	for i in 0..iterations {
		key = (key * subject) % 20201227;
	}
	key
}

#[test]
fn testp1() {
	assert_eq!(day25_part1("5764801
17807724"), 14897079)
}