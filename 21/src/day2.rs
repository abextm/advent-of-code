#[aoc(day2, part1)]
fn day2_part1(input: &str) -> i32 {
	let mut pos: (i32, i32) = (0, 0);
	input.trim().split("\n")
		.for_each(|p| {
			let mut parts = p.split(" ");
			let dir = parts.next().unwrap();

			let arg: i32 = parts.next().unwrap().parse().unwrap();
			match dir {
				"forward" => pos.0 += arg,
				"down" => pos.1 += arg,
				"up" => pos.1 -= arg,
				o => panic!("{}", o),
			}
		});
		return pos.0 * pos.1;
}

#[aoc(day2, part2)]
fn day2_part2(input: &str) -> i32 {
	let mut aim = 0;
	let mut pos: (i32, i32) = (0, 0);
	input.trim().split("\n")
		.for_each(|p| {
			let mut parts = p.split(" ");
			let dir = parts.next().unwrap();

			let arg: i32 = parts.next().unwrap().parse().unwrap();
			match dir {
				"forward" => {
					pos.0 += arg;
					pos.1 += aim * arg;
				},
				"down" => aim += arg,
				"up" => aim -= arg,
				o => panic!("{}", o),
			}
		});
		return pos.0 * pos.1;
}

#[test]
fn testd2p1() {
	assert_eq!(150, day2_part1("forward 5
down 5
forward 8
up 3
down 8
forward 2"))
}