use regex::Regex;
use crate::grid::Ve;

fn orient(a: Ve<2>, b: Ve<2>, c: Ve<2>) -> isize {
	(b - a).cross(&(c - a))
}

fn intersect(a: (Ve<2>, Ve<2>), b: (Ve<2>, Ve<2>)) -> Ve<2> {
	let oa = orient(b.0, b.1, a.0);
	let ob = orient(b.0, b.1, a.1);
	(a.0 * Ve::from(ob) - a.1 * Ve::from(oa)) / Ve::from(ob - oa)
}

#[aoc(part1 = 31623, part2 = 93209116744825)]
fn solve(input: &str, part1: bool) -> impl std::fmt::Debug {
	let re = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)\nButton B: X\+([0-9]+), Y\+([0-9]+)\nPrize: X=([0-9]+), Y=([0-9]+)").unwrap();
	let add = if part1 { 0 } else { 10000000000000isize };
	input.trim().split("\n\n").map(|input| {
		let [a, b, target] = re.captures(input)
			.expect(input)
			.iter()
			.skip(1)
			.map(|s| s.unwrap().as_str().parse::<isize>().unwrap())
			.array_chunks()
			.map(|c| Ve(c))
			.next_chunk()
			.unwrap();

		let target = target + Ve::from(add);

		let pt = intersect((Ve::zero(), a), (target - b, target));
		if pt % a == Ve::zero() && (target - pt) % b == Ve::zero() {
			let dd = Ve([pt[0] / a[0], (target[0] - pt[0]) / b[0]]);
			if !part1 || dd.max_element() <= 100 {
				return dd[0] * 3 + dd[1];
			}
		}
		0
	}).sum::<isize>()
}

#[aoc(part1 = 480)]
const EX: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";