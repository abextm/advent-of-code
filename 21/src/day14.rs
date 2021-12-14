const NONE: u8 = b' ';

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
	let mut iter = input.trim().split("\n\n");
	let mut poly = iter.next().unwrap().bytes().collect::<Vec<_>>();
	let mut rules = [NONE; 256*256];
	for line in iter.next().unwrap().lines() {
		let mut i = line.split(" -> ");
		let mut rl = i.next().unwrap().bytes();
		let rule = (rl.next().unwrap() as usize) << 8 | rl.next().unwrap() as usize;
		let val = i.next().unwrap().bytes().next().unwrap();
		rules[rule] = val;
	}

	for _iteration in 0..10 {
		let mut next = Vec::with_capacity(poly.len() * 2);
		for i in 0..(poly.len()-1) {
			let l = poly[i];
			let r = poly[i + 1];
			let val = (l as usize) << 8 | r as usize;
			next.push(l);
			next.push(rules[val]);
		}
		next.push(*poly.last().unwrap());
		poly = next;
	}
	let mut by_el = [0usize; 256];
	for v in poly {
		by_el[v as usize] += 1;
	}
	let mut els = by_el.iter().cloned().enumerate()
		.filter(|&(_, v)| v > 0)
		.collect::<Vec<_>>();
	els.sort_by_key(|&(_, v)| v);
	els.last().unwrap().1 - els.first().unwrap().1
}


#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
	let mut iter = input.trim().split("\n\n");
	let mut poly = iter.next().unwrap().bytes().collect::<Vec<_>>();
	let mut rules = [NONE; 256*256];
	for line in iter.next().unwrap().lines() {
		let mut i = line.split(" -> ");
		let mut rl = i.next().unwrap().bytes();
		let rule = (rl.next().unwrap() as usize) << 8 | rl.next().unwrap() as usize;
		let val = i.next().unwrap().bytes().next().unwrap();
		rules[rule] = val;
	}

	for _iteration in 0..40 {
		let mut next = Vec::with_capacity(poly.len() * 2);
		for i in 0..(poly.len()-1) {
			let l = poly[i];
			let r = poly[i + 1];
			let val = (l as usize) << 8 | r as usize;
			next.push(l);
			next.push(rules[val]);
		}
		next.push(*poly.last().unwrap());
		poly = next;
	}
	let mut by_el = [0usize; 256];
	for v in poly {
		by_el[v as usize] += 1;
	}
	let mut els = by_el.iter().cloned().enumerate()
		.filter(|&(_, v)| v > 0)
		.collect::<Vec<_>>();
	els.sort_by_key(|&(_, v)| v);
	els.last().unwrap().1 - els.first().unwrap().1
}