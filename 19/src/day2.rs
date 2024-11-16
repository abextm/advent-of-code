use crate::vm;

#[aoc(part1=6327510)]
fn day2_part1(input: &str) -> i64 {
	let mut state = vm::new_from_str(input).unwrap();
	state.eval_0(&[12, 2]).unwrap()
}

#[aoc(part2=4112)]
fn day2_part2(input: &str) -> i64 {
	let state = vm::new_from_str(input).unwrap();
	
	for range in 0 ..= 999_999 {
		for shift in 0 ..= range {
			let a = range - shift;
			let b = shift;
			let rval = state.clone().eval_0(&[a, b]);
			if rval.ok() == Some(19690720) {
				return (100 * a) + b;
			}
		}
	}

	unreachable!();
}