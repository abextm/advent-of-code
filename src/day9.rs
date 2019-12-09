use crate::vm;

#[aoc(day9, part1)]
fn day9_part1(input: &str) -> i64 {
	vm::new_from_str(input).unwrap()
		.with_input_vec(&[1])
		.next().unwrap().unwrap()
}

#[aoc(day9, part2)]
fn day9_part2(input: &str) -> i64 {
	vm::new_from_str(input).unwrap()
	.with_input_vec(&[2])
	.next().unwrap().unwrap()
}
