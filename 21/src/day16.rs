struct BitIter<'a> {
	vec: &'a [u8],
	offset: usize,
	end: usize,
}

impl<'a> Iterator for BitIter<'a> {
	type Item = bool;
	fn next(&mut self) -> Option<bool> {
		if self.offset >= self.end {
			None
		} else {
			let b = self.vec[self.offset / 8];
			let v = b >> (7 - (self.offset % 8)) & 1;
			self.offset += 1;
			Some(v != 0)
		}
	}
}

impl<'a> BitIter<'a> {
	fn take_bits(&mut self, n: usize) -> Option<usize> {
		let mut o = 0;
		for i in 0..n {
			if let Some(v) = self.next() {
				o = o << 1 | v as usize;
			} else {
				if i == 0 {
					return None
				} else {
					panic!();
				}
			}
		}
		Some(o)
	}

	fn slice(&mut self, bits: usize) -> Self {
		let offset = self.offset;
		self.offset += bits;
		BitIter{
			vec: self.vec,
			offset,
			end: self.offset,
		}
	}

	fn read_packet(&mut self) -> Option<Packet> {
		let ver = self.take_bits(3)?;
		let id = self.take_bits(3).unwrap();
		if id == 4 {
			let mut v = 0;
			loop {
				let cont = self.next().unwrap();
				v = v << 4 | self.take_bits(4).unwrap();
				if !cont {
					break;
				}
			}
			return Some(Packet::Literal(ver, v))
		}
		let lmode = self.next().unwrap();
		Some(Packet::Operator(ver, id, if lmode {
			let cnt = self.take_bits(11).unwrap();
			(0..cnt)
				.map(|_| self.read_packet().unwrap())
				.collect()
		} else {
			let len = self.take_bits(15).unwrap();
			let mut sub = self.slice(len);
			let mut out = Vec::new();
			while let Some(p) = sub.read_packet() {
				out.push(p);
			}
			out
		}))
	}
}

#[derive(Eq, PartialEq, Debug)]
enum Packet {
	Literal(usize, usize),
	Operator(usize, usize, Vec<Packet>),
}

impl Packet {
	fn sum_version(&self) -> usize {
		match self {
			Packet::Literal(ver, _) => *ver,
			Packet::Operator(ver, _, children) => *ver + children.iter().map(|x| x.sum_version()).sum::<usize>(),
		}
	}

	fn value(&self) -> usize {
		match self {
			Packet::Literal(_, val) => *val,
			Packet::Operator(_, typ, children) => {
				match *typ {
					0 => children.iter().map(|x| x.value()).sum::<usize>(),
					1 => children.iter().map(|x| x.value()).fold(1, |acc, v| acc *v),
					2 => children.iter().map(|x| x.value()).min().unwrap(),
					3 => children.iter().map(|x| x.value()).max().unwrap(),
					5 => (children[0].value() > children[1].value()) as usize,
					6 => (children[0].value() < children[1].value()) as usize,
					7 => (children[0].value() == children[1].value()) as usize,
					_ => panic!(),
				}
			}
		}
	}
}

fn parse(input: &str) -> Packet {
	let bytes: Vec<_> = input.trim().as_bytes()
		.windows(2)
		.step_by(2)
		.map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
		.collect();
	let mut iter = BitIter {
		vec: &bytes,
		end: bytes.len() * 8,
		offset: 0,
	};
	iter.read_packet().unwrap()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
	parse(input).sum_version()
}

#[aoc(day16, part2)]
fn part2(input: &str) -> usize {
	parse(input).value()
}

#[test]
fn test() {
	assert_eq!(parse("D2FE28"), Packet::Literal(6, 2021));
	assert_eq!(parse("EE00D40C823060"), Packet::Operator(7, 3, vec![
		Packet::Literal(2, 1),
		Packet::Literal(4, 2),
		Packet::Literal(1, 3),
	]));
	assert_eq!(parse("A0016C880162017C3686B18A3D4780").sum_version(), 31);
	assert_eq!(parse("9C0141080250320F1802104A08").value(), 1);
}