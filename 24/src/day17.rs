struct VM {
	combo: [u64; 16],
	prog: Vec<Op>,
	source: Vec<u8>,
}

struct VMExec<'a> {
	combo: [u64; 16],
	prog: &'a [Op],
}

struct Op {
	xor: u8,
	shr: u8,
	lhs: u8,
	res: u8,
	and: i8,
}

const REG_A: u8 = 4 + 8;
const REG_B: u8 = 5 + 8;
const REG_C: u8 = 6 + 8;
const REG_IP: u8 = 7 + 8;

const RES_OUT: u8 = 0xFF;


impl VM {
	fn parse(input: &str) -> VM {
		let [a, b, c, prog] = input.lines()
			.filter(|n| n.len() > 0)
			.map(|l| l.split(": ").last().unwrap())
			.next_chunk()
			.unwrap();

		let [a, b, c] = [a, b, c].map(|p| p.parse::<u64>().unwrap());
		let source: Vec<u8> = prog
			.split(",")
			.map(|n| n.parse().unwrap())
			.collect();

		let prog = source.iter()
			.cloned()
			.array_chunks()
			.map(|[opcode, literal]| {
				let combo = literal + 8;
				let literal = literal;

				match opcode {
					0 => Op{xor: 0, shr: combo, and: -1, lhs: REG_A, res: REG_A},
					1 => Op{xor: literal, shr: 0, and: -1, lhs: REG_B, res: REG_B},
					2 => Op{xor: 0, shr: 0, and: 7, lhs: combo, res: REG_B},
					3 => Op{xor: 0, shr: 1, and: -1, lhs: literal, res: REG_IP},
					4 => Op{xor: REG_C, shr: 0, and: -1, lhs: REG_B, res: REG_B},
					5 => Op{xor: 0, shr: 0, and: 7, lhs: combo, res: RES_OUT},
					6 => Op{xor: 0, shr: combo, and: -1, lhs: REG_A, res: REG_B},
					7 => Op{xor: 0, shr: combo, and: -1, lhs: REG_A, res: REG_C},
					_ => panic!(),
				}
			})
			.collect();

		VM {
			combo: [0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, a, b, c, 0],
			prog,
			source,
		}
	}

	fn exec(&self) -> VMExec<'_> {
		VMExec {
			combo: self.combo,
			prog: &self.prog,
		}
	}
}

impl<'a> Iterator for VMExec<'a> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(op) = self.prog.get(self.combo[REG_IP as usize] as usize) {
			let mut v = self.combo[op.lhs as usize] ^ self.combo[op.xor as usize];
			v >>= self.combo[op.shr as usize];
			v &= (op.and as i64) as u64;

			self.combo[REG_IP as usize] += 1;

			if op.res < REG_IP {
				self.combo[op.res as usize] = v;
			} else if op.res == REG_IP {
				if self.combo[REG_A as usize] != 0 {
					self.combo[op.res as usize] = v;
				}
			} else if op.res == RES_OUT {
				return Some(v as u8);
			}
		}
		None
	}
}



#[aoc(part1="\"6,1,6,4,2,4,7,3,5\"")]
fn part1(input: &str) -> impl std::fmt::Debug {
	VM::parse(input).exec().map(|n| (n + b'0') as char).intersperse(',').collect::<String>()
}

#[aoc(part2=202975183645226)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let vm = VM::parse(input);

	let mut a = 0;

	loop {
		let mut e = vm.exec();
		e.combo[REG_A as usize] = a;
		let mut p = vm.source.iter().cloned();
		let mut last_bad = None;
		for i in 0.. {
			let vmv = e.next();
			let sv = p.next();
			if sv == None {
				if vmv != None {
					panic!(); // overshot
				}
				break;
			}
			if vmv != sv {
				last_bad = Some(i);
			}
		}
		if let Some(i) = last_bad {
			a += 1 << (i * 3);
		} else {
			return a;
		}
	}
}

#[aoc(part1="\"4,6,3,5,6,3,5,2,1,0\"")]
const EX: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
