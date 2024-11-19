use crate::vm;

#[aoc(part1=144)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let vm = {
		let mut vm = vm::new_from_str(input)
			.unwrap();
		vm.consume_input()
			.unwrap();
		vm
	};
	let mut n = 0;
	for x in 0..50 {
		let mut vm_x = vm.clone().with_input([x].into_iter());
		vm_x.consume_input().unwrap();
		for y in 0..50 {
			let mut vm_y = vm_x.clone().with_input([y].into_iter());
			n += vm_y.next().unwrap().unwrap();
		}
	}

	n
}

#[aoc(part2=13561537)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let vm = {
		let mut vm = vm::new_from_str(input)
			.unwrap();
		vm.consume_input()
			.unwrap();
		vm.without_input()
	};

	let width = 100 - 1;

	let mut y_min = 0;
	for x in width.. {
		let mut vm_xhi = vm.clone().with_input([x].into_iter());
		vm_xhi.consume_input().unwrap();
		for y in y_min.. {
			let mut vm_y = vm_xhi.clone().with_input([y].into_iter());
			if vm_y.next().unwrap().unwrap() == 1 {
				y_min = y;
				break;
			}
		}

		let mut vm_xlo = vm.clone().with_input([x - width].into_iter());
		vm_xlo.consume_input().unwrap();

		if vm_xlo.clone().with_input([y_min + width].into_iter()).next().unwrap().unwrap() == 0 {
			continue;
		}

		if vm_xlo.clone().with_input([y_min].into_iter()).next().unwrap().unwrap() == 0 {
			continue;
		}


		return (x - width) * 10000 + y_min;
	}

	panic!();
}