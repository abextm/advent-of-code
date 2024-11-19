use std::error;
use std::io;
use std::iter;
use std::str;
use strum_macros::Display;

#[derive(Debug, Display, Clone)]
pub enum EvalError {
	UnknownOpcode(i64),
	InvalidAddressingMode(i64),
	IllegalAddressingMode(i64),
	EndOfInput,
	UnexpectedResult(EvalResult),
}

impl error::Error for EvalError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		None
	}
}

#[derive(Debug, Display, PartialEq, Clone)]
pub enum EvalResult {
	Ok,
	Output(i64),
	Return,
}

lazy_static! {
	static ref PARAMETER_DIVS: [i64; 4] = crate::taken::generate(1, |x, _i| x * 10);
}

pub struct State<I: Iterator<Item = i64>> {
	pub memory: Vec<i64>,
	pub pc: usize,
	pub base: isize,
	pub input: I,
}

mod op {
	pub const ADD: i64 = 1;
	pub const MUL: i64 = 2;
	pub const IN: i64 = 3;
	pub const OUT: i64 = 4;
	pub const JNZ: i64 = 5;
	pub const JEZ: i64 = 6;
	pub const LT: i64 = 7;
	pub const EQ: i64 = 8;
	pub const BASE: i64 = 9;
	pub const RET: i64 = 99;
}

mod amode {
	pub const ABSOLUTE_PTR: i64 = 0;
	pub const IMMEDIATE: i64 = 1;
	pub const RELATIVE_PTR: i64 = 2;
}

pub fn new(memory: Vec<i64>) -> State<iter::Empty<i64>> {
	State {
		memory: memory,
		pc: 0,
		base: 0,
		input: iter::empty(),
	}
}

pub fn new_from_str(s: &str) -> Result<State<iter::Empty<i64>>, <i64 as std::str::FromStr>::Err> {
	let mut vec: Vec<i64> = Vec::new();
	for seg_ws in s.split(',') {
		let seg = seg_ws.trim();
		if seg.len() > 0 {
			vec.push(seg.parse()?);
		}
	}
	Ok(new(vec))	
}

pub fn new_from_stream(
	stream: impl io::BufRead,
) -> Result<State<iter::Empty<i64>>, Box<dyn error::Error>> {
	let mut vec: Vec<i64> = Vec::new();
	for seg_br in stream.split(b',') {
		let seg_b = seg_br?;
		let seg = str::from_utf8(&seg_b)?.trim();
		if seg.len() > 0 {
			vec.push(seg.parse()?);
		}
	}
	Ok(new(vec))
}

impl<I: Iterator<Item = i64> + Clone> Clone for State<I> {
	fn clone(&self) -> Self {
		State {
			memory: self.memory.clone(),
			pc: self.pc,
			base: self.base,
			input: self.input.clone(),
		}
	}
}

impl<I: Iterator<Item = i64>> State<I> {
	pub fn with_input<J: Iterator<Item = i64>>(self, input: J) -> State<J> {
		State {
			memory: self.memory,
			pc: self.pc,
			base: self.base,
			input: input,
		}
	}

	pub fn with_input_vec<'a>(self, input: &'static [i64]) -> State<impl Iterator<Item = i64>> {
		self.with_input(input.iter().cloned())
	}

	pub fn without_input(self) -> State<iter::Empty<i64>> {
		State {
			memory: self.memory,
			pc: self.pc,
			base: self.base,
			input: iter::empty(),
		}
	}

	pub fn eval_0(&mut self, args: &[i64]) -> Result<i64, EvalError> {
		self.memory[1..1 + args.len()].clone_from_slice(args);
		self.run()?;
		Ok(self.memory[0])
	}

	pub fn consume_input(&mut self) -> Result<(), EvalError> {
		loop {
			match self.single_step() {
				Ok(EvalResult::Ok) => continue,
				Ok(res) => return Err(EvalError::UnexpectedResult(res)),
				Err(EvalError::EndOfInput) => return Ok(()),
				Err(v) => return Err(v),
			}
		}
	}

	pub fn run(&mut self) -> Result<(), EvalError> {
		while self.single_step()? != EvalResult::Return {}
		Ok(())
	}

	pub fn single_step(&mut self) -> Result<EvalResult, EvalError> {
		if self.memory.len() < self.pc {
			return Err(EvalError::UnknownOpcode(0));
		}
		let opcode_packed = self.memory[self.pc];
		let opcode = opcode_packed % 100;
		match opcode {
			op::ADD => {
				*self.deref_mut(3)? = self.deref(1)? + self.deref(2)?;
				self.pc += 4;
			}
			op::MUL => {
				*self.deref_mut(3)? = self.deref(1)? * self.deref(2)?;
				self.pc += 4;
			}
			op::IN => {
				*self.deref_mut(1)? = self.input.next().ok_or(EvalError::EndOfInput)?;
				self.pc += 2;
			}
			op::OUT => {
				let val = self.deref(1)?;
				self.pc += 2;
				return Ok(EvalResult::Output(val));
			}
			op::JNZ => {
				if self.deref(1)? != 0 {
					self.pc = self.deref(2)? as usize;
				} else {
					self.pc += 3;
				}
			}
			op::JEZ => {
				if self.deref(1)? == 0 {
					self.pc = self.deref(2)? as usize;
				} else {
					self.pc += 3;
				}
			}
			op::LT => {
				*self.deref_mut(3)? = (self.deref(1)? < self.deref(2)?) as i64;
				self.pc += 4;
			}
			op::EQ => {
				*self.deref_mut(3)? = (self.deref(1)? == self.deref(2)?) as i64;
				self.pc += 4;
			}
			op::BASE => {
				self.base +=  self.deref(1)? as isize;
				self.pc += 2;
			}
			op::RET => return Ok(EvalResult::Return),
			_ => return Err(EvalError::UnknownOpcode(opcode)),
		}

		Ok(EvalResult::Ok)
	}

	fn param_value(&self, parameter: i64) -> Result<(i64, i64), EvalError> {
		let opcode_packed = self.memory[self.pc];
		let addr_mode = (opcode_packed / PARAMETER_DIVS[parameter as usize]) % 10;
		let index = self.pc + parameter as usize;
		if index >= self.memory.len() {
			Ok((addr_mode, 0))
		} else {
			Ok((addr_mode, self.memory[index]))
		}
	}

	fn deref(&self, parameter: i64) -> Result<i64, EvalError> {
		let (addr_mode, val) = self.param_value(parameter)?;
		let ptr = match addr_mode {
			amode::ABSOLUTE_PTR => val as usize,
			amode::RELATIVE_PTR => (val as isize + self.base) as usize,
			amode::IMMEDIATE => return Ok(val),
			_ => return Err(EvalError::InvalidAddressingMode(addr_mode)),
		};
		if ptr >= self.memory.len() {
			Ok(0)
		} else {
			Ok(self.memory[ptr])
		}
	}

	fn deref_mut<'memory>(&'memory mut self, parameter: i64) -> Result<&'memory mut i64, EvalError> {
		let (addr_mode, val) = self.param_value(parameter)?;
		let ptr = match addr_mode {
			amode::ABSOLUTE_PTR => val as usize,
			amode::RELATIVE_PTR => (val as isize + self.base) as usize,
			amode::IMMEDIATE => return Err(EvalError::IllegalAddressingMode(addr_mode)),
			_ => return Err(EvalError::InvalidAddressingMode(addr_mode)),
		};
		if ptr >= self.memory.len() {
			self.memory.extend((self.memory.len()..=ptr).map(|_x| 0));
		}
		Ok(&mut self.memory[ptr])
	}
}

impl<I: Iterator<Item = i64>> Iterator for State<I> {
	type Item = Result<i64, EvalError>;
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.single_step() {
				Ok(EvalResult::Output(v)) => return Some(Ok(v)),
				Ok(EvalResult::Ok) => continue,
				Ok(EvalResult::Return) => return None,
				Err(e) => return Some(Err(e)),
			}
		}
	}
}

#[test]
fn example_d2_1() {
	assert_eq!(
		3500,
		new([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].to_vec())
			.eval_0(&[])
			.unwrap()
	);
}

#[test]
fn example_d5_1() {
	let mut s = new([1002, 4, 3, 4, 33].to_vec());
	s.single_step().unwrap();
	assert_eq!(99, s.memory[4]);
}

#[cfg(test)]
fn test_inout(mem: &[i64], inp: &'static [i64], outp: &[i64]) {
	assert_eq!(
		new(mem.to_vec())
			.with_input_vec(inp)
			.map(|r| r.unwrap())
			.collect::<Vec<_>>(),
		outp
	);
}

#[test]
fn inout() {
	let x = 9;
	test_inout(
		&[op::IN, x, op::MUL + 100, 2, x, x, op::OUT, x, op::RET, 0],
		&[44],
		&[88],
	);
}

#[test]
fn example_d5_p2_cmp() {
	test_inout(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[8], &[1]);
	test_inout(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[2], &[0]);

	test_inout(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[2], &[1]);
	test_inout(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[8], &[0]);
}

#[test]
fn example_d5_p2_jmp() {
	test_inout(
		&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
		&[44],
		&[1],
	);
	test_inout(
		&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
		&[0],
		&[0],
	);
}

#[test]
fn example_d5_p2_big() {
	let prog = [
		3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
		1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
		1, 46, 98, 99,
	];
	test_inout(&prog, &[7], &[999]);
	test_inout(&prog, &[8], &[1000]);
	test_inout(&prog, &[9], &[1001]);
}

#[test]
fn example_d9_quine() {
	let prog = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
	test_inout(&prog, &[], &prog);
}

#[test]
fn example_d9_bignum() {
	test_inout(&[104,1125899906842624,99], &[], &[1125899906842624])
}