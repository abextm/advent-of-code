use crate::vm;

#[aoc(day5, part1)]
fn day5_part1(input: &str) -> i64 {
	day5(input, 1)
}

#[aoc(day5, part2)]
fn day5_part2(input: &str) -> i64 {
	day5(input, 5)
}

fn day5(input: &str, mod_id: i64) -> i64 {
	let mut vals = vm::new_from_str(input)
		.unwrap()
		.with_input([mod_id].iter().map(|x| *x))
		.collect::<Vec<_>>();

	let retval = vals.pop().unwrap();
	for (id, val) in vals.into_iter().enumerate() {
		match val {
			Err(e) => panic!(e),
			Ok(0) => (),
			Ok(val) => println!("{} is {}", id, val),
		}
	}
	retval.unwrap()
}