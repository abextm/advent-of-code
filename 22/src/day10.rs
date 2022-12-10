#[aoc(day10, part1)]
fn day10_part1(input: &str) -> isize {
	let mut x: isize = 1;
	let mut cycle: isize = 0;
	let mut sum = 0;
	for line in input.lines() {
		let mut instr = line.split(' ');
		let op = instr.next().unwrap();
		let num_cycles = match op {
			"addx" => 2,
			_ => 1,
		};
		cycle += num_cycles;
		if (cycle + 20) % 40 < num_cycles {
			println!("{} {}", (cycle + 20) - (cycle % 40) - 20, x);
			sum += x * (cycle - (cycle % 20));
		}
		match op {
			"addx" => {
				x += instr.next().unwrap().parse::<isize>().unwrap();
			},
			"noop" => (),
			v => panic!("{}", v),
		}
	}

	sum
}


#[aoc(day10, part2)]
fn day10_part2(input: &str) -> usize {
	let mut x: isize = 1;
	let mut cycle: isize = 0;
	for line in input.lines() {
		let mut instr = line.split(' ');
		let op = instr.next().unwrap();
		let num_cycles = match op {
			"addx" => 2,
			_ => 1,
		};
		for _ in 0..num_cycles {
			cycle+=1;
			let xcoord = cycle % 40;
			print!("{}", if (x..x+3).contains(&xcoord) { '#' } else { ' ' });
			if cycle % 40 == 0 {
				println!();
			}
		}
		match op {
			"addx" => {
				x += instr.next().unwrap().parse::<isize>().unwrap();
			},
			"noop" => (),
			v => panic!("{}", v),
		}
	}

	0
}

#[test]
fn test() {
	assert_eq!(day10_part1(EXAMPLE_1), 13140);
}

#[cfg(test)]
const EXAMPLE_1: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";