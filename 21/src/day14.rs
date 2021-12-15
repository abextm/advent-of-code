use std::mem::swap;

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
	solve(input, 10)
}
#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
	solve(input, 40)
}

fn solve(input: &str, iterations: isize) -> usize {
	let mut iter = input.trim().split("\n\n");
	let poly = iter.next().unwrap().bytes().map(|b| b - b'A').collect::<Vec<_>>();
	let mut rules = [128; 32*32];
	for line in iter.next().unwrap().lines() {
		let mut i = line.split(" -> ");
		let mut rl = i.next().unwrap().bytes().map(|b| b - b'A');
		let rule = (rl.next().unwrap() as usize) << 5 | rl.next().unwrap() as usize;
		let val = i.next().unwrap().bytes().next().unwrap() - b'A';
		rules[rule] = val;
	}

	let mut pairs = [0usize; 32 * 32];
	for i in 0..(poly.len()-1) {
		let l = poly[i];
		let r = poly[i + 1];
		pairs[(l as usize) << 5 | r as usize] += 1;
	}

	let mut pairs2 = [0; 32 * 32];
	let mut prev = &mut pairs;
	let mut next = &mut pairs2;
	for _ in 0..iterations {
		next.fill(0);
		for (pair, &v) in prev.iter().enumerate() {
			let m = rules[pair] as usize;
			let l = pair >> 5;
			let r = pair & 31;
			if v > 0 {
				next[l << 5 | m] += v;
				next[m << 5 | r] += v;
			}
		}
		swap(&mut prev, &mut next);
	}

	let mut by_el = [0; 32];
	by_el[*poly.last().unwrap() as usize] += 1;

	for (pair, &v) in prev.iter().enumerate() {
		by_el[(pair >> 5)] += v;
	}


	let mut els = by_el.iter().cloned().enumerate()
		.filter(|&(_, v)| v > 0)
		.collect::<Vec<_>>();
	els.sort_by_key(|&(_, v)| v);
	els.last().unwrap().1 - els.first().unwrap().1
}
