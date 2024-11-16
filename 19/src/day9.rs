use crate::vm;

#[aoc(part1=2351176124, part2=73110)]
fn day9_part1(input: &str, part: i64) -> i64 {
	vm::new_from_str(input).unwrap()
		.with_input([part].into_iter())
		.next().unwrap().unwrap()
}