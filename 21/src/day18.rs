use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone)]
enum Number {
	Number(usize),
	Pair(Box<(Number, Number)>),
}

impl Debug for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Number::Number(v) => write!(f, "{}", *v),
			Number::Pair(b) => write!(f, "[{:?},{:?}]", b.0, b.1),
		}
	}
}

#[derive(PartialEq, Eq)]
enum ExplodeResult {
	Exploding(Option<usize>, Option<usize>),
	Same,
}

impl Number {
	fn parse(input: &str) -> Self {
		Number::parse_inner(&mut input.bytes().peekable()).unwrap()
	}
	fn parse_inner(input: &mut std::iter::Peekable<std::str::Bytes>) -> Option<Self> {
		match input.next() {
			Some(b'[') => {
				let a = Self::parse_inner(input).unwrap();
				if Some(b',') != input.next() {
					panic!();
				}
				let b = Self::parse_inner(input).unwrap();
				if Some(b']') != input.next() {
					panic!();
				}
				Some(Number::Pair((a, b).into()))
			},
			Some(a) if a>= b'0' && a <= b'9' => Some(Number::Number((a - b'0') as usize)),
			None => None,
			_ => panic!()
		}
	}

	fn reduce(mut self) -> Self {
		loop {
			while self.explode() {}
			if !self.split() {
				return self
			}
		}
	}

	fn explode(&mut self) -> bool {
		self.explode_inner(0) != ExplodeResult::Same
	}

	fn explode_inner(&mut self, depth: usize) -> ExplodeResult {
		match self {
			Number::Number(_) => ExplodeResult::Same,
			Number::Pair(b) => {
				if depth >= 4 {
					if let &(Number::Number(l), Number::Number(r)) = b.as_ref() {
						*self = Number::Number(0);
						return ExplodeResult::Exploding(Some(l), Some(r));
					}
				}
				match b.0.explode_inner(depth + 1) {
					ExplodeResult::Same => (),
					ExplodeResult::Exploding(l, r) => {
						if let Some(v) = r {
							b.1.add_left(v);
						}
						return ExplodeResult::Exploding(l, None);
					}
				};
				match b.1.explode_inner(depth + 1) {
					ExplodeResult::Same => (),
					ExplodeResult::Exploding(l, r) => {
						if let Some(v) = l {
							b.0.add_right(v);
						}
						return ExplodeResult::Exploding(None, r);
					}
				};
				return ExplodeResult::Same;
			}
		}
	}

	fn add_left(&mut self, ov: usize) {
		match self {
			Number::Number(v) => *v += ov,
			Number::Pair(b) => b.0.add_left(ov),
		}
	}
	fn add_right(&mut self, ov: usize) {
		match self {
			Number::Number(v) => *v += ov,
			Number::Pair(b) => b.1.add_right(ov),
		}
	}

	fn split(&mut self) -> bool {
		match self {
			Number::Number(v) => {
				if *v < 10 {
					false
				} else {
					let v = *v;
					let a= v / 2;
					let b = v - a;
					*self = Number::Pair((Number::Number(a), Number::Number(b)).into());
					true
				}
			},
			Number::Pair(b) => b.0.split() || b.1.split(),
		}
	}

	fn magnitude(&self) -> usize {
		match self {
			Number::Number(v) => *v,
			Number::Pair(p) => p.0.magnitude() * 3 + p.1.magnitude() * 2,
		}
	}
}

#[aoc(day18, part1)]
fn part1(input: &str) -> usize {
	let mut it = input.lines().map(|x| Number::parse(x));
	let v = it.next().unwrap();
	it.fold(v, |acc, v| Number::Pair((acc, v).into()).reduce()).magnitude()
}

#[aoc(day18, part2)]
fn part2(input: &str) -> usize {
	let input: Vec<_> = input.lines().map(|x| Number::parse(x)).collect();
	
	(0..input.len()).flat_map(|a| (0..input.len()).filter_map(move |b| if a != b {Some((a, b))} else {None}))
		.map(|(a, b)| Number::Pair((input[a].clone(), input[b].clone()).into()).reduce().magnitude())
		.max()
		.unwrap()
}

#[cfg(test)]
fn assert_reduces_to(a: &str, b: &str) {
	assert_eq!(Number::parse(a).reduce(), Number::parse(b));
}

#[test]
fn test() {
	assert_reduces_to("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
	assert_reduces_to("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
	assert_reduces_to("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
	assert_reduces_to("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
	assert_reduces_to("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[cfg(test)]
const EXAMPLE_P2: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

#[test]
fn test_p2() {
	assert_eq!(part2(EXAMPLE_P2), 3993);
}