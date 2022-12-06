#[aoc(day6, part1)]
fn day6_part1(input: &str) -> usize {
	solve(input, 4)
}

#[aoc(day6, part2)]
fn day6_part2(input: &str) -> usize {
	solve(input, 14)
}

fn solve(input: &str, dup: usize) -> usize {
	let input = input.trim().as_bytes();
	'outer: for i in dup..input.len() {
		for j in 0..dup {
			for k in 0..dup {
				if j != k && input[i - j] == input [i - k] {
					continue 'outer;
				}
			}
		}

		return i + 1;
	}

	panic!();
}

#[test]
fn test_p1() {
	let test = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
	assert_eq!(day6_part1(test), 11);
	assert_eq!(day6_part2(test), 26);
}