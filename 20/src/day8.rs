#[derive(Clone, Copy, Debug)]
pub enum Opcode {
	Nop,
	Acc,
	Jmp,
}

#[derive(Clone, Copy, Debug)]
pub struct Instr {
	opcode: Opcode,
	operand: i64,
}

#[derive(Clone)]
pub struct VM<'op> {
	pc: usize,
	acc: i64,
	text: &'op[Instr],
}

impl<'op> VM<'op> {
	pub fn parse(input: &str) -> Vec<Instr> {
		input
			.trim()
			.split("\n")
			.map(|line| {
				let parts = line.split(" ").collect::<Vec<_>>();
				let opcode = match parts[0] {
					"nop" => Opcode::Nop,
					"acc" => Opcode::Acc,
					"jmp" => Opcode::Jmp,
					v => panic!("bad instr \"{}\"", v),
				};
				Instr {
					opcode,
					operand: parts[1].parse().unwrap(),
				}
			})
		.collect()
	}

	pub fn new(text: &'op[Instr]) -> VM<'op> {
		VM {
			text,
			pc: 0,
			acc: 0,
		}
	}

	pub fn single_step(&mut self) {
		let instr = &self.text[self.pc];
		match instr.opcode {
			Opcode::Nop => (),
			Opcode::Acc => self.acc += instr.operand,
			Opcode::Jmp => self.pc = (self.pc as i64 + instr.operand - 1) as usize,
		}

		self.pc += 1;
	}

	pub fn is_done(&self) -> bool {
		self.pc == self.text.len()
	}
}

#[aoc(day8, part1)]
fn day8_part1(input: &str) -> i64 {
	let text = VM::parse(input);
	let mut vm = VM::new(&text);

	let mut done = Vec::new();
	done.resize_with(vm.text.len(), || false);

	while !vm.is_done() {
		if done[vm.pc] {
			return vm.acc;
		}
		done[vm.pc] = true;
		vm.single_step();
	}
	panic!();
}


#[aoc(day8, part2)]
fn day8_part2(input: &str) -> i64 {
	let mut text = VM::parse(input);
	
	for i in 0..text.len() {
		let v = {
			let vp = &mut text[i].opcode;
			let v = *vp;
			*vp = match v {
				Opcode::Nop => Opcode::Jmp,
				Opcode::Jmp => Opcode::Nop,
				Opcode::Acc => continue,
			};
			v
		};
		
		match test(&text) {
			Some(v) => return v,
			None => (),
		}
		text[i].opcode = v;
	}
	panic!();
}

fn test(text: &[Instr]) -> Option<i64> {
	let mut vm = VM::new(&text);

	let mut done = Vec::new();
	done.resize_with(vm.text.len(), || false);

	while !vm.is_done() {
		if done[vm.pc] {
			return None
		}
		done[vm.pc] = true;
		vm.single_step();
	}

	Some(vm.acc)
}
