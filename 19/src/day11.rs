use crate::taken::TakeN;
use crate::vm;
use std::collections::HashMap;
use std::sync::mpsc;

#[aoc(day11, part1)]
fn day11_part1(input: &str) -> usize {
	day11(input, false).len()
}

#[aoc(day11, part2)]
fn day11_part2(input: &str) -> String {
	let panel = day11(input, true);

	let lx = panel.keys().map(|x| x.0).min().unwrap();
	let ux = panel.keys().map(|x| x.0).max().unwrap() + 1;
	let ly = panel.keys().map(|x| x.1).min().unwrap();
	let uy = panel.keys().map(|x| x.1).max().unwrap() + 1;

	let mut s = String::with_capacity(((ux - lx) + 1) * (uy - ly) + 1);
	s.push('\n');
	for y in ly..uy {
		for x in lx..ux {
			s.push(match panel.get(&(x, y)) {
				Some(0) => '█',
				Some(1) =>  '░',
				None => ' ',
				_ => '.',
			});
		}
		s.push('\n')
	}
	s
}

fn day11(input: &str, part2: bool) -> HashMap::<(usize, usize), i64> {
	let mut pos = (0, 0);
	let mut direction = 0;
	let mut panel = HashMap::<(usize, usize), i64>::new();

	if part2 {
		panel.insert(pos, 1);
	}

	let (send, recv) = mpsc::channel();
	let mut vm = vm::new_from_str(input)
		.unwrap()
		.with_input(recv.into_iter());
	loop {
		let panel = panel.entry(pos).or_insert(0);
		send.send(*panel).unwrap();
		if let Some(v) = vm.take_n() {
			let [color, turn]: [_; 2] = v;
			*panel = color.unwrap();
			direction += if turn.unwrap() == 0 { -1 } else { 1 };
			direction += 4;
			direction %= 4;
			match direction {
				0 => pos.1 -= 1,
				1 => pos.0 += 1,
				2 => pos.1 += 1,
				3 => pos.0 -= 1,
				_ => unreachable!(),
			}
		} else {
			break;
		}
	}
	panel
}
