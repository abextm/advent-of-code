#[aoc(day12, part1)]
fn day12_part1(input: &str) -> f64 {
	let mut x = 0.0;
	let mut y = 0.0;
	let mut bearing = 90.0;
	for line in input.trim().split("\n") {
		let amnt: f64 = line[1..].parse().unwrap();
		match &line[0..1] {
			"N" => y -= amnt,
			"S" => y += amnt,
			"E" => x += amnt,
			"W" => x -= amnt,
			"F" => {
				x += amnt * (bearing / 180.0 * std::f64::consts::PI).sin();
				y -= amnt * (bearing / 180.0 * std::f64::consts::PI).cos();
			}
			"R" => bearing += amnt,
			"L" => bearing -= amnt,
			v => panic!("{}", v),
		}
	}

	x.abs() + y.abs()
}

#[test]
fn p1() {
	assert_eq!(day12_part1("F10\nN3\nF7\nR90\nF11"), 25.0);
}

#[aoc(day12, part2)]
fn day12_part2(input: &str) -> f64 {
	let mut xw = 10.0;
	let mut yw = -1.0;
	let mut x = 0.0;
	let mut y = 0.0;
	for line in input.trim().split("\n") {
		let amnt: f64 = line[1..].parse().unwrap();
		match &line[0..1] {
			"N" => yw -= amnt,
			"S" => yw += amnt,
			"E" => xw += amnt,
			"W" => xw -= amnt,
			"F" => {
				x += amnt * xw;
				y -= amnt * yw;
			}
			"R" => {
				let ang = amnt / 180.0 * std::f64::consts::PI;
				let xw2 = ang.cos() * xw - ang.sin() * yw;
				let yw2 = ang.sin() * xw + ang.cos() * yw;
				xw = xw2;
				yw = yw2;
			}
			"L" => {
				let ang = -amnt / 180.0 * std::f64::consts::PI;
				let xw2 = ang.cos() * xw - ang.sin() * yw;
				let yw2 = ang.sin() * xw + ang.cos() * yw;
				xw = xw2;
				yw = yw2;
			},
			v => panic!("{}", v),
		}
	}

	x.abs() + y.abs()
}

#[test]
fn p2() {
	assert_eq!(day12_part2("F10\nN3\nF7\nR90\nF11"), 286.0);
}