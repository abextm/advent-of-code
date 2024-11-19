use crate::vm;

#[aoc(part1=19350375, part2=1143990055)]
fn solve(input: &str, part1: bool) -> impl std::fmt::Debug {
	let prog = if part1 {
		r#"
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
"#
	} else {
		r"
OR E J
OR H J
AND D J
NOT A T
NOT T T
AND B T
AND C T
NOT T T
AND T J
RUN
"
	};

	let mut dpy = String::new();
	for v in vm::new_from_str(input).unwrap()
		.with_input(prog[1..].chars().map(|x| x as i64)) {
		let v = v.unwrap();
		if v >= 128 {
			return v;
		}
		dpy.push(v as u8 as char);
	}

	println!("{}", dpy);
	panic!();
}
