use crate::vm;
use std::sync::mpsc;

#[aoc(day7, part1)]
fn day7_part1(input: &str) -> i32 {
	day7(input, false)
}

#[aoc(day7, part2)]
fn day7_part2(input: &str) -> i32 {
	day7(input, true)
}

fn day7(input: &str, part2: bool) -> i32 {
	let template = vm::new_from_str(input).unwrap();

	let stage_templates: Vec<_> = if !part2 { (0..5) } else { (5..10) }
		.map(|n| {
			let mut vm = template.clone().with_input(n..=n);
			match vm.run() {
				Err(vm::EvalError::EndOfInput) => vm.without_input(),
				v => panic!("Didn't hit eoi? {:?}", v),
			}
		})
		.collect();

	const NUM_AMPS: usize = 5;
	
	let mut pow = 1;
	let powers = (0..NUM_AMPS)
		.map(|_| {
			let r = pow;
			pow *= NUM_AMPS;
			r
		})
		.collect::<Vec<_>>();

	(0..pow)
		.filter_map(|n| {
			let vec = powers
				.iter()
				.map(|p| (n / p) % NUM_AMPS)
				.collect::<Vec<_>>();
			let mut vecmut = vec.clone();
			vecmut.sort();
			vecmut.dedup();
			if vecmut.len() == 5 {
				Some(vec)
			} else {
				None
			}
		})
		.map(|phases| {
			let (send, recv) = mpsc::channel();
			let root: Box<dyn Iterator<Item = i32>> = Box::new(recv.iter());
			let mut chain = root;
			for phase in phases {
				chain = Box::new(
					stage_templates[phase]
						.clone()
						.with_input(chain)
						.map(Result::unwrap),
				)
			}
			if !part2 {
				send.send(0).unwrap();
				chain.next().unwrap()
			} else {
				send.send(0).unwrap();
				for sig in chain {
					send.send(sig).unwrap();
				}
				recv.recv().unwrap()
			}
		})
		.max()
		.unwrap()
}
