#[aoc(day10, part1)]
fn day10_part1(input: &str) -> usize {
	input.trim().lines().filter_map(|line| {
		let mut stack = Vec::new();
		for char in line.chars() {
			match char {
				'[' => stack.push(']'),
				'{' => stack.push('}'),
				'(' => stack.push(')'),
				'<' => stack.push('>'),
				']' | '}' | ')' | '>' => {
					let should_be = stack.pop();
					if Some(char) != should_be {
						return Some(match char {
							')' => 3,
							']' => 57,
							'}' => 1197,
							'>' => 25137,
							_ => panic!(),
						})
					}
				}
				_ => panic!("{}", char),
			}
		}
		None
	}).sum()
}

#[aoc(day10, part2)]
fn day10_part2(input: &str) -> usize {
	let mut scores: Vec<usize> = input.trim().lines().filter_map(|line| {
		let mut stack = Vec::new();
		for char in line.chars() {
			match char {
				'[' => stack.push(']'),
				'{' => stack.push('}'),
				'(' => stack.push(')'),
				'<' => stack.push('>'),
				']' | '}' | ')' | '>' => {
					let should_be = stack.pop();
					if Some(char) != should_be {
						return None
					}
				}
				_ => panic!("{}", char),
			}
		}
		Some(stack.iter().rev().map(|x| 
			match *x {
				')' => 1,
				']' => 2,
				'}' => 3,
				'>' => 4,
				_ => panic!(),
		}).fold(0usize, |a, b| (a * 5) + b))
		}).collect();
	scores.sort();
	scores[scores.len() / 2]
}


const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

#[test]
fn test_part1() {
	assert_eq!(26397, day10_part1(EXAMPLE));
}
#[test]
fn test_part2() {
	assert_eq!(288957, day10_part2(EXAMPLE));
}