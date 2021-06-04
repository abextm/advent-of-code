#[aoc(day13, part1)]
fn day13_part1(input: &str) -> i64 {
	let mut input = input.trim().split("\n");
	let start: i64 = input.next().unwrap().parse().unwrap();
	let busses = input.next().unwrap().split(",").map(|p| p.parse::<i64>()).collect::<Vec<_>>();
	let mut min = 9999999;
	let mut minid = -1;
	for bus in busses {
		if let Ok(bus) = bus {
			let v = bus - (start % bus);
			if min > v {
				min = v;
				minid = bus;
			}
		}
	}
	
	min * minid
}

#[aoc(day13, part2)]
fn day13_part2(input: &str) -> usize {
	let mut input = input.trim().split("\n");
	input.next();
	let busses = input.next().unwrap().split(",").enumerate().filter_map(|(i, p)| match p.parse::<usize>() {
		Ok(v) => Some((i, v)),
		Err(_) => None,
	}).collect::<Vec<_>>();
	let mut step = busses[0].1;
	let mut i = 0;
	for &(index, bus) in busses.iter().skip(1) {
		loop {
			let d = (i+index) % bus;
			if d == 0 {
				step *= bus;
				println!("{} {} {} {}", i, step, index, i - index);
				break;
			}
			i += step;
		}
	}
	
	i
}

#[test]
fn testp2() {
	//assert_eq!(day13_part2("x\n7,13,x,x,59,x,31,19"), 1068781);
	assert_eq!(day13_part2("x\n17,x,13,19"), 3417);
}