use core::panic;
use std::rc::Rc;
use std::cell::RefCell;
use opencl3 as cl;

fn index(c: &str) -> Value {
	match c {
		"x" => Value::Index(0),
		"y" => Value::Index(1),
		"z" => Value::Index(2),
		"w" => Value::Index(3),
		v => Value::Immediate(v.parse().expect(v)),
	}
}

#[derive(PartialEq, Eq)]
enum Value {
	Index(usize),
	Immediate(isize),
}
struct Insn {
	opcode: usize,
	a: Value,
	b: Value,
}
impl Value {
	fn get(&self, mem: &[SymVal; 4]) -> SymVal {
		match self {
			&Value::Immediate(v) => SymVal::Constant(v),
			&Value::Index(v) => mem[v].clone(),
		}
	}
	fn iget(&self, mem: &[isize; 4]) -> isize {
		match self {
			&Value::Immediate(v) => v,
			&Value::Index(v) => mem[v],
		}
	}
	fn iset(&self, mem: &mut [isize; 4], val: isize) {
		match self {
			&Value::Immediate(_) => panic!(),
			&Value::Index(v) => mem[v] = val,
		}
	}
}

fn interpret(mut input: &[isize], prog: &[Insn]) -> [isize; 4] {
	let mut val = [0isize; 4];
	for insn in prog {
		let v = match insn.opcode {
			INP => {
				let v = input[0];
				input = &input[1..];
				v
			},
			ADD => insn.a.iget(&val) + insn.b.iget(&val),
			MUL => insn.a.iget(&val) * insn.b.iget(&val),
			DIV => insn.a.iget(&val) / insn.b.iget(&val),
			MOD => insn.a.iget(&val) % insn.b.iget(&val),
			EQL => (insn.a.iget(&val) == insn.b.iget(&val)) as isize,
			v => panic!("{}", v),
		};
		insn.a.iset(&mut val, v);
	}
	val
}

const INP: usize = 1;
const ADD: usize = 2;
const MUL: usize = 3;
const DIV: usize = 4;
const MOD: usize = 5;
const EQL: usize = 6;
const NEQ: usize = 7;

fn execute(prog: &[Insn]) -> [SymVal; 4] {
	let mut val = [SymVal::Constant(0), SymVal::Constant(0), SymVal::Constant(0), SymVal::Constant(0)];
	let mut input = 0;
	for (index, insn) in prog.iter().enumerate() {
		let v = match insn.opcode {
			INP => {
				let v = SymVal::Input(input);
				input += 1;
				v
			},
			op => {
				let a = insn.a.get(&val);
				let b = insn.b.get(&val);

				for v in [&a, &b] {
					if let SymVal::Result(v) = v {
						*v.borrow().uses.borrow_mut() += 1;
					}
				}

				SymVal::Result(Rc::new(RefCell::new(SymExpr{
					opcode: op,
					a,
					b,
					index,
					uses: 0.into(),
					name: 0.into(),
					range: None.into(),
				})))
			},
		};
		if let Value::Index(idx) = insn.a {
			val[idx] = v;
		}
	}
	val
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Range {
	min: isize,
	max: isize,
}

pub struct Fmt<F>(pub F) where F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result;
impl<F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result> std::fmt::Debug for Fmt<F> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		(self.0)(f)
	}
}

fn optimize(v: &mut SymVal) -> (bool, Range) {
	let mut it  = 0;
	while let SymVal::Result(rc) = v.clone() {
		it += 1;
		if it > 1000 {
			panic!("{:?}", v);
		}
		let mut rc = rc.borrow_mut();
		let op= rc.opcode;
		let a = &rc.a;
		let b = &rc.b;
		if let (SymVal::Constant(a), SymVal::Constant(b)) = (a, b) {
			*v = SymVal::Constant(match op {
				ADD => a + b,
				MUL => a * b,
				DIV => a / b,
				MOD => a % b,
				EQL => (a == b) as isize,
				NEQ => (a != b) as isize,
				v => panic!("{}", v),
			});
			if it > 999 {
				panic!("{:?}", v);
			}
			continue;
		}
		if op == MUL && (&SymVal::Constant(0) == a || &SymVal::Constant(0) == b) {
			*v = SymVal::Constant(0);
			continue;
		}
		if op == ADD && (&SymVal::Constant(0) == a || &SymVal::Constant(0) == b) {
			*v = match (a, b) {
				(SymVal::Constant(0), b) => b.clone(),
				(a, SymVal::Constant(0)) => a.clone(),
				_ => panic!(),
			};
			continue;
		}
		if (op == DIV || op == MUL) && &SymVal::Constant(1) == b {
			*v = a.clone();
			continue;
		}
		if op == MUL && &SymVal::Constant(1) == a {
			*v = b.clone();
			continue;
		}
		if op == EQL && &SymVal::Constant(0) == b {
			if let &SymVal::Result(ia) = &a {
				if ia.borrow().opcode == EQL {
					drop(a);
					drop(b);
					let v = SymExpr{
						opcode: NEQ,
						a: ia.borrow().a.clone(),
						b: ia.borrow().b.clone(),
						index: rc.index,
						uses: rc.uses.clone(),
						name: 0.into(),
						range: None.into(),
					};
					*rc = v;
					continue;
				}
			}
		}
		let (ach, arange) = optimize(&mut rc.a);
		let (bch, brange) = optimize(&mut rc.b);
		if ach || bch {
			continue;
		}
		if op == NEQ || op == EQL {
			if arange.min > brange.max || brange.min > arange.max {
				*v = SymVal::Constant((op == NEQ) as isize);
			}
		}
		let range = match op {
			ADD => Range{min: arange.min + brange.min, max: arange.max + brange.max},
			MUL => Range{min: arange.min * brange.min, max: arange.max * brange.max},
			DIV => Range{min: arange.min / brange.max, max: arange.max / brange.min},
			MOD => Range{min: arange.min % brange.min, max: arange.max.min(brange.max)},
			EQL => Range{min: 0, max: 1},
			NEQ => Range{min: 0, max: 1},
			_ => panic!()
		};

		if range.min == range.max {
			*v = SymVal::Constant(range.min);
			continue;
		}

		*rc.range.borrow_mut() = Some(range);

		return (it > 1, range);
	}
	let range = match &*v {
		&SymVal::Constant(v) => Range{min: v, max: v},
		&SymVal::Input(_) => Range{min: 1, max: 9},
		_ => panic!(),
	};
	(it > 1, range)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SymVal {
	Constant(isize),
	Input(usize),
	Result(Rc<RefCell<SymExpr>>),
}

#[derive(Debug, PartialEq, Eq)]
struct SymExpr {
	opcode: usize,
	a: SymVal,
	b: SymVal,
	index: usize,
	uses: RefCell<usize>,
	name: RefCell<usize>,
	range: RefCell<Option<Range>>,
}

fn find_multivars(out: &mut Vec<Rc<RefCell<SymExpr>>>, v: &SymVal) {
	if let SymVal::Result(rv) = v {
		let v = rv.borrow();
		if *v.uses.borrow() > 1 {
			out.push(rv.clone());
			*v.uses.borrow_mut() = 0;
		}
		for v in [&v.a, &v.b] {
			find_multivars(out, &v);
		}
	}
}

fn print_val(f: &mut std::fmt::Formatter, v: &SymVal, force: bool) -> std::fmt::Result {
	match v {
		&SymVal::Constant(n) => write!(f, "{}", n),
		&SymVal::Input(n) => write!(f, "input[{}]", n),
		SymVal::Result(rc) => print_expr(f, rc, force),
	}
}

fn print_expr(f: &mut std::fmt::Formatter, rrc: &Rc<RefCell<SymExpr>>, force: bool) -> std::fmt::Result {
	let rc = rrc.borrow();
	let is_var = *rc.uses.borrow() == 0;
	if is_var && !force {
		return write!(f, "var{:02}", *rc.name.borrow());
	}
	let op = match rc.opcode {
		ADD => "+",
		MUL => "*",
		DIV => "/",
		MOD => "%",
		EQL => "==",
		NEQ => "!=",
		_ => panic!(),
	};
	let is_cmp = rc.opcode == EQL || rc.opcode == NEQ;
	if is_var {
		write!(f, "ulong var{:02} = ", *rc.name.borrow());
	};
	if !is_var || is_cmp {
		write!(f, "(")?;
	}
	print_val(f, &rc.a, false)?;
	write!(f, " {} ", op)?;
	print_val(f, &rc.b, false)?;
	if !is_var || is_cmp {
		write!(f, ")")?;
	}
	if is_cmp {
		//write!(f, " as isize");
	}
	if is_var {
		write!(f, ";\n")?;
	}
	Ok(())
}

#[aoc(day24, part1)]
fn part1(input: &str) -> u64 {
	solve(input, false)
}
#[aoc(day24, part2)]
fn part2(input: &str) -> u64 {
	solve(input, true)
}

fn solve(input: &str, is_min: bool) -> u64 {
	let prog: Vec<_> = input.lines().map(|line| {
		let parts: Vec<_> = line.split(" ").collect();
		let opcode = match parts[0] {
			"inp" => INP,
			"add" => ADD,
			"mul" => MUL,
			"div" => DIV,
			"mod" => MOD,
			"eql" => EQL,
			v => panic!("{}", v),
		};
		Insn {
			opcode,
			a: index(parts[1]),
			b: index(parts.get(2).unwrap_or(&"0")),
		}
	}).collect();

	let mut wanted = execute(&prog)[2].clone();
	optimize(&mut wanted);
	let mut mvs = Vec::new();
	find_multivars(&mut mvs, &wanted);
	mvs.sort_by_key(|x| x.borrow().index);
	for (index, mv) in mvs.iter().enumerate() {
		*mv.borrow().name.borrow_mut() = index + 1;
	}

	let func = format!("bool calculate(ulong *input) {{\n{:?}\n}}", Fmt(|f| {
		for v in mvs.iter() {
			print_expr(f, &v, true)?;
		}
		print_val(f,&wanted, true)?;
		write!(f, "return var00 == 0;")
	}));

	println!("{}", func);

	let lop = if is_min {
		"#define LOOP(N) for (model[N] = 1; model[N] <= 9; model[N]++)
#define DIR +=
"
	} else {
		"#define LOOP(N) for (model[N] = 9; model[N] >= 1; model[N]--)
#define DIR -=
"
	};

	let kernel = "
__kernel void part1(
	__global __read_write ulong* output,
	ulong start
) {
	ulong model[14] = {9};
	start DIR get_global_id(0) * 10000000;
	for (int i = 13; i >= 0; i--) {
		model[i] = start % 10;
		start /= 10;
	}

	output[get_global_id(0)] = 0;

	LOOP(7)
	LOOP(8)
	LOOP(9)
	LOOP(10)
	LOOP(11)
	LOOP(12)
	LOOP(13)
	{
		if (calculate(&model)) {
			ulong val = 0;
			bool failed = false;
			for(int i = 0; i < 14; i++) {
				val *= 10;
				val += model[i];
				failed |= model[i] == 0;
			}
			if (!failed) {
				output[get_global_id(0)] = val;
				return;
			}
		}
	}
}
";

	let dev = cl::device::Device::new(*cl::device::get_all_devices(cl::device::CL_DEVICE_TYPE_GPU)
		.unwrap()
		.first()
		.unwrap());
	let ctx = cl::context::Context::from_device(&dev).unwrap();
	let clq = cl::command_queue::CommandQueue::create(&ctx, ctx.default_device(), 0).unwrap();
	let clprog = cl::program::Program::create_and_build_from_source(&ctx, &(lop.to_owned() + &func + kernel), "").unwrap();
	let knl = cl::kernel::Kernel::create(&clprog, "part1").unwrap();

	let outbuf = cl::memory::Buffer::<cl::types::cl_ulong>::create(&ctx, cl::memory::CL_MEM_WRITE_ONLY | cl::memory::CL_MEM_HOST_READ_ONLY, 1024, std::ptr::null_mut()).unwrap();

	let it: Box<dyn Iterator<Item=u64>> = if is_min {
		Box::new(11111111111111..=99999999999999u64)
	} else {
		Box::new((0..=99999999999999u64).rev())
	};
	for start in it.step_by(10000000 * 1024) {
		let ev = cl::kernel::ExecuteKernel::new(&knl)
			.set_arg(&outbuf)
			.set_arg(&start)
			.set_global_work_size(1024)
			.enqueue_nd_range(&clq)
			.unwrap();
		let mut out = [0 as cl::types::cl_ulong; 1024];
		clq.enqueue_read_buffer(&outbuf, cl::types::CL_BLOCKING, 0, &mut out, &[ev.get()]).unwrap();
		if let Some(max) = out.iter().copied().filter(|&x| x != 0).next() {
			let mut model = [0; 14];
			let mut acc = max;
			for i in (0..14).rev() {
				model[i] = (acc % 10) as isize;
				acc /= 10;
			}
			let exec_v = interpret(&model, &prog);
			println!("{:?}", exec_v);

			return max;
		}
		println!("{}", start);
	}

	0
}	
