use std::collections::VecDeque;
use std::iter::Empty;
use crate::vm;

struct NIC {
	vm: vm::State<Empty<i64>>,
	write_buf: Vec<i64>,
}

struct Queue {
	q: VecDeque<i64>,
	idle: bool,
}

impl Queue {
	fn push(&mut self, x: i64, y: i64) {
		self.q.push_back(x);
		self.q.push_back(y);
		self.idle = false;
	}

	fn take(&mut self) -> i64 {
		match self.q.pop_front() {
			Some(v) => v,
			None => {
				self.idle = true;
				-1
			},
		}
	}
}

#[aoc(part1=14834, part2=10215)]
fn solve(input: &str, part1: bool) -> impl std::fmt::Debug {
	let mut vm = vm::new_from_str(input).unwrap();
	vm.consume_input().unwrap();
	let mut vms = Vec::new();
	for i in 0..50 {
		let mut vm = vm.clone()
			.with_input([i].into_iter());
		vm.consume_input().unwrap();
		vms.push(NIC {
			vm: vm.without_input(),
			write_buf: Vec::with_capacity(3),
		});
	}

	let mut nat_last_y = None;
	let mut nat = (0i64, 0i64);
	let mut queues = (0..vms.len()).map(|_| Queue {
		q: VecDeque::new(),
		idle: false,
	}).collect::<Vec<_>>();
	loop {
		let mut all_idle = true;
		for (i, vm) in vms.iter_mut().enumerate() {
			match vm.vm.single_step() {
				Ok(vm::EvalResult::Ok) => (),
				Ok(vm::EvalResult::Output(v)) => {
					vm.write_buf.push(v);
					queues[i].idle = false;
					if vm.write_buf.len() == 3 {
						let addr = vm.write_buf[0] as usize;
						if addr == 255 {
							if part1 {
								return vm.write_buf[2];
							} else {
								nat = (vm.write_buf[1], vm.write_buf[2]);

							}
						} else {
							queues[addr].push(vm.write_buf[1], vm.write_buf[2])
						}
						vm.write_buf.clear();
					}
				},
				Ok(vm::EvalResult::Return) => panic!(),
				Err(vm::EvalError::EndOfInput) => {
					let v = queues[i].take();
					let mut vmv = vm::new(Vec::new());
					std::mem::swap(&mut vmv, &mut vm.vm);
					let mut vmv = vmv.with_input([v].into_iter());
					assert_eq!(vmv.single_step(), Ok(vm::EvalResult::Ok));
					let mut vmv = vmv.without_input();
					std::mem::swap(&mut vmv, &mut vm.vm);
				}
				Err(e) => panic!("{}", e),
			}
			all_idle &= queues[i].idle;
		}

		if all_idle {
			if nat_last_y == Some(nat.1) {
				return nat.1;
			}

			nat_last_y = Some(nat.1);
			queues[0].push(nat.0, nat.1);
		}
	}
}