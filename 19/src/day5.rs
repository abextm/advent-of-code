use crate::vm;

#[aoc(part1=6761139, part2=9217546)]
fn day5(input: &str, part: i64) -> i64 {
	let mod_id = match part {
		1 => 1,
		2 => 5,
		_ => panic!(),
	};
	let mut vals = vm::new_from_str(input)
		.unwrap()
		.with_input([mod_id].iter().cloned())
		.collect::<Vec<_>>();

	let retval = vals.pop().unwrap();
	for (id, val) in vals.into_iter().enumerate() {
		match val {
			Err(e) => panic!("{:?}", e),
			Ok(0) => (),
			Ok(val) => println!("{} is {}", id, val),
		}
	}
	retval.unwrap()
}