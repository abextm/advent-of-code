use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Number {
	Number(u8),
	Open,
	Close,
}

#[derive(PartialEq, Eq, Clone)]
struct Snail (Vec<Number>);

impl Debug for Snail {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for v in &self.0 {
			match v {
				&Number::Number(v) => write!(f, "{},", v)?,
				&Number::Open => write!(f, "[")?,
				&Number::Close => write!(f, "]")?,
			}
		}
		Ok(())
	}
}

impl Snail {
	fn parse(input: &str) -> Self {
		Snail(input.bytes().filter_map(|x| {
			match x {
				b',' => None,
				b'[' => Some(Number::Open),
				b']' => Some(Number::Close),
				v if v >= b'0' && v <= b'9' => Some(Number::Number(v - b'0')),
				_ => panic!(),
			}
		}).collect())
	}

	fn add(&self, r: &Snail) -> Snail {
		let mut v = Vec::with_capacity(self.0.len() + r.0.len() + 2);
		v.push(Number::Open);
		v.extend_from_slice(&self.0);
		v.extend_from_slice(&r.0);
		v.push(Number::Close);
		let mut v = Snail(v);
		v.reduce();
		v
	}

	fn reduce(&mut self) {
		while self.explode() || self.split() {}
	}

	fn explode(&mut self) -> bool {
		let mut depth = 0usize;
		for i in 0..self.0.len() {
			match &self.0[i] {
				&Number::Open => depth += 1,
				&Number::Close => depth -= 1,
				&Number::Number(l) if depth > 4 => {
					if let &Number::Number(r) = &self.0[i + 1] {
						self.0.splice(i - 1..=i + 2, [Number::Number(0)]);
						self.add_left(i - 1, l);
						self.add_right(i - 1, r);
						return true
					}
				}
				_ => (),
			}
		}
		false
	}

	fn add_right(&mut self, start: usize, val: u8) {
		for i in (start+1)..self.0.len() {
			if let Number::Number(n) = &mut self.0[i] {
				*n += val;
				return
			}
		}
	}
	fn add_left(&mut self, start: usize, val: u8) {
		for i in (0..start).rev() {
			if let Number::Number(n) = &mut self.0[i] {
				*n += val;
				return
			}
		}
	}

	fn split(&mut self) -> bool {
		for i in 0..self.0.len() {
			if let Number::Number(n) = self.0[i] {
				if n >= 10 {
					let a= n / 2;
					let b = n - a;
					let new = [
						Number::Open,
						Number::Number(a),
						Number::Number(b),
						Number::Close,
					];
					self.0.splice(i..=i, new);
					return true
				}
			}
		}
		false
	}

	fn magnitude(&self) -> usize {
		let mut shunt = Vec::with_capacity(16);
		for v in &self.0 {
			match v {
				&Number::Open => (),
				&Number::Number(n) => shunt.push(n as usize),
				&Number::Close => {
					let r = shunt.pop().unwrap();
					let l = shunt.pop().unwrap();
					shunt.push(l * 3 + r * 2)
				}
			}
		}
		assert_eq!(shunt.len(), 1);
		shunt[0]
	}
}

#[aoc(day18, part1)]
fn part1(input: &str) -> usize {
	let mut it = input.lines().map(|x| Snail::parse(x));
	let v = it.next().unwrap();
	it.fold(v, |acc, v| acc.add(&v)).magnitude()
}

#[aoc(day18, part2)]
fn part2(input: &str) -> usize {
	let input: Vec<_> = input.lines().map(|x| Snail::parse(x)).collect();
	
	(0..input.len()).flat_map(|a| (0..input.len()).filter_map(move |b| if a != b {Some((a, b))} else {None}))
		.map(|(a, b)| input[a].add(&input[b]).magnitude())
		.max()
		.unwrap()
}

#[cfg(test)]
fn assert_reduces_to(a: &str, b: &str) {
	let mut v = Snail::parse(a);
	v.reduce();
	assert_eq!(v, Snail::parse(b));
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