use std::error;
use std::io;
use std::iter;
use std::str;
use strum_macros::Display;

#[derive(Debug, Display, Clone)]
pub enum EvalError {
	UnknownOpcode(i32),
	IllegalArgument(i32),
	IllegalDereference(i32),
	InvalidAddressingMode(i32),
	IllegalAddressingMode(i32),
	EndOfInput,
	EOF,
}

impl error::Error for EvalError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		None
	}
}

#[derive(Debug, Display, PartialEq)]
pub enum EvalResult {
	Ok,
	Output(i32),
	Return,
}

lazy_static! {
	static ref PARAMETER_DIVS: [i32; 4] = crate::util::generate(1, |x, _i| x * 10);
}

pub struct State<I: Iterator<Item = i32>> {
	pub memory: Vec<i32>,
	pub pc: usize,
	pub input: I,
}

mod op {
	pub const ADD: i32 = 1;
	pub const MUL: i32 = 2;
	pub const IN: i32 = 3;
	pub const OUT: i32 = 4;
	pub const JNZ: i32 = 5;
	pub const JEZ: i32 = 6;
	pub const LT: i32 = 7;
	pub const EQ: i32 = 8;
	pub const RET: i32 = 99;
}

mod amode {
	pub const ABSOLUTE_PTR: i32 = 0;
	pub const IMMEDIATE: i32 = 1;
}

pub fn new(memory: Vec<i32>) -> State<iter::Empty<i32>> {
	State {
		memory: memory,
		pc: 0,
		input: iter::empty(),
	}
}

pub fn new_from_str(s: &str) -> Result<State<iter::Empty<i32>>, <i32 as std::str::FromStr>::Err> {
	let mut vec: Vec<i32> = Vec::new();
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
) -> Result<State<iter::Empty<i32>>, Box<dyn error::Error>> {
	let mut vec: Vec<i32> = Vec::new();
	for seg_br in stream.split(b',') {
		let seg_b = seg_br?;
		let seg = str::from_utf8(&seg_b)?.trim();
		if seg.len() > 0 {
			vec.push(seg.parse()?);
		}
	}
	Ok(new(vec))
}

impl<I: Iterator<Item = i32> + Clone> Clone for State<I> {
	fn clone(&self) -> Self {
		State {
			memory: self.memory.clone(),
			pc: self.pc,
			input: self.input.clone(),
		}
	}
}

impl<I: Iterator<Item = i32>> State<I> {
	pub fn with_input<J: Iterator<Item = i32>>(self, input: J) -> State<J> {
		State {
			memory: self.memory,
			pc: self.pc,
			input: input,
		}
	}

	pub fn with_input_vec<'a>(self, input: &'static [i32]) -> State<impl Iterator<Item = i32>> {
		self.with_input(input.clone().iter().map(|x| *x))
	}

	pub fn without_input(self) -> State<iter::Empty<i32>> {
		State {
			memory: self.memory,
			pc: self.pc,
			input: iter::empty(),
		}
	}

	pub fn eval_0(&mut self, args: &[i32]) -> Result<i32, EvalError> {
		self.memory[1..1 + args.len()].clone_from_slice(args);
		self.run()?;
		Ok(self.memory[0])
	}

	pub fn run(&mut self) -> Result<(), EvalError> {
		while self.single_step()? != EvalResult::Return {}
		Ok(())
	}

	pub fn single_step(&mut self) -> Result<EvalResult, EvalError> {
		if self.memory.len() < self.pc {
			return Err(EvalError::EOF);
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
				*self.deref_mut(3)? = (self.deref(1)? < self.deref(2)?) as i32;
				self.pc += 4;
			}
			op::EQ => {
				*self.deref_mut(3)? = (self.deref(1)? == self.deref(2)?) as i32;
				self.pc += 4;
			}
			op::RET => return Ok(EvalResult::Return),
			_ => return Err(EvalError::UnknownOpcode(opcode)),
		}

		Ok(EvalResult::Ok)
	}

	fn deref(&self, parameter: i32) -> Result<i32, EvalError> {
		let opcode_packed = self.memory[self.pc];
		let addr_mode = (opcode_packed / PARAMETER_DIVS[parameter as usize]) % 10;
		match addr_mode {
			amode::ABSOLUTE_PTR => {
				let index = self.pc + parameter as usize;
				if index >= self.memory.len() {
					Err(EvalError::IllegalArgument(parameter))
				} else {
					let ptr = self.memory[index];
					if ptr as usize >= self.memory.len() {
						Err(EvalError::IllegalDereference(ptr))
					} else {
						Ok(self.memory[ptr as usize])
					}
				}
			}
			amode::IMMEDIATE => {
				let index = self.pc + parameter as usize;
				if index >= self.memory.len() {
					Err(EvalError::IllegalArgument(parameter))
				} else {
					Ok(self.memory[index])
				}
			}
			_ => Err(EvalError::InvalidAddressingMode(addr_mode)),
		}
	}

	fn deref_mut<'memory>(&'memory mut self, parameter: i32) -> Result<&'memory mut i32, EvalError> {
		let opcode_packed = self.memory[self.pc];
		let addr_mode = (opcode_packed / PARAMETER_DIVS[parameter as usize]) % 10;
		match addr_mode {
			amode::ABSOLUTE_PTR => {
				let index = self.pc + parameter as usize;
				if index >= self.memory.len() {
					Err(EvalError::IllegalArgument(parameter))
				} else {
					let ptr = self.memory[index];
					if ptr as usize >= self.memory.len() {
						Err(EvalError::IllegalDereference(ptr))
					} else {
						Ok(&mut self.memory[ptr as usize])
					}
				}
			}
			amode::IMMEDIATE => Err(EvalError::IllegalAddressingMode(addr_mode)),
			_ => Err(EvalError::InvalidAddressingMode(addr_mode)),
		}
	}
}

impl<I: Iterator<Item = i32>> Iterator for State<I> {
	type Item = Result<i32, EvalError>;
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
fn test_inout(mem: &[i32], inp: &'static [i32], outp: &[i32]) {
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
