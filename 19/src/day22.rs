use std::ops::Rem;
use num::ToPrimitive;
use num::traits::Euclid;

#[derive(Debug)]
enum Move{
	DealWithIncrement(i64),
	NewStack,
	Cut(i64),
}

impl Move {
	fn forward(&self, v: i64, md: i64) -> i64 {
		match self {
			Move::NewStack => md - 1 - v,
			Move::DealWithIncrement(step) => (v * step) % md,
			Move::Cut(n) => (v - n).rem_euclid(md),
		}
	}

	fn forward_fn(&self, f: &mut Fn) {
		match self {
			Move::NewStack => {
				f.mul(-1);
				f.add(-1);
			},
			&Move::DealWithIncrement(step) => {
				f.mul(step);
			}
			&Move::Cut(n) => {
				f.add(-n);
			}
		}
	}
}

fn parse(input: &str) -> Vec<Move> {
	let mut moves = Vec::new();
	for line in input.trim().lines() {
		if let Some(v) =  line.strip_prefix("deal with increment ") {
			moves.push(Move::DealWithIncrement(v.parse().unwrap()));
		} else if let Some(v) = line.strip_prefix("cut ") {
			moves.push(Move::Cut(v.parse().unwrap()));
		} else if line == "deal into new stack" {
			moves.push(Move::NewStack);
		} else {
			panic!("unexpected line: {}", line);
		}
	}
	moves
}

#[aoc(part1=2519)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let moves = parse(input);

	let size = 10007;
	let in_val = 2019;

	let mut v = in_val;
	let mut f = Fn::new(size);
	for mov in moves {
		v = mov.forward(v, size);
		mov.forward_fn(&mut f);
		if f.eval(in_val) != v {
			panic!("{:?}", mov);
		}
	}

	assert_eq!(f.reverse().eval(v), in_val);

	v
}
// f(x) = (x + add) * mul (mod md)
#[derive(Debug)]
struct Fn {
	md: i64,
	add: i64,
	mul: i64,
}

impl Fn {
	fn add(&mut self, v: i64) {
		// f(x) = (x + a) * m + v
		let v = self.modmul(v, self.modinv(self.mul));
		self.add = (self.add + v) % self.md;
	}

	fn mul(&mut self, v: i64) {
		// f(x) = (x + a) * m * v
		self.mul = self.modmul(self.mul, v);
	}

	fn modmul(&self, a: i64, b: i64) -> i64 {
		(a as i128 * b as i128).rem_euclid(self.md as i128) as i64
	}

	fn modinv(&self, v: i64) -> i64 {
		let o = num::BigInt::from(v).modinv(&self.md.into());
		if let Some(v) = o {
			v.to_i64().unwrap()
		} else {
			panic!("{}", v);
		}
	}

	fn eval(&self, v: i64) -> i64 {
		self.modmul(v + self.add, self.mul)
	}

	fn reverse(&self) -> Self {
		// f(x) = (x * mul^-1) - add
		let mut f = Fn::new(self.md);
		f.mul(f.modinv(self.mul));
		f.add(-self.add);
		f
	}

	fn new(modulo: i64) -> Fn {
		Fn {
			md: modulo,
			add: 0,
			mul: 1,
		}
	}

	fn modpow(&self, base: i64, n: i64) -> i64 {
		num::BigInt::from(base).modpow(&n.into(), &self.md.into()).to_i64().unwrap()
	}

	fn eval_iterated(&self, x: i64, n: i64) -> i64 {
		// convert to f(x) = mul * x + add
		let add = self.modmul(self.add, self.mul);

		let mulN = self.modpow(self.mul, n);
		(self.modmul(mulN, x) + self.modmul(self.modmul(mulN - 1, self.modinv(self.mul - 1)), add)).rem_euclid(self.md)
	}
}

#[aoc(part2)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let moves = parse(input);

	let mut fun = Fn::new(119315717514047);
	for mov in moves {
		mov.forward_fn(&mut fun);
	}
	
	let rev = fun.reverse();
	
	let mut v = 2020;
	for i in 1..5 {
		v = rev.eval(v);
		let it = rev.eval_iterated(2020, i);
		assert_eq!(it, v, "{}", i);
	}

	rev.eval_iterated(2020, 101741582076661)
}
