use std::ops::RangeInclusive;

type Range1D = RangeInclusive<isize>;
type Range3D = [Range1D; 3];

fn r3d_union(mut a: Range3D, b: &Range3D) -> Option<Range3D> {
	for dim in 0..a.len() {
		let v =*a[dim].start().max(b[dim].start())..=*a[dim].end().min(b[dim].end());
		if v.is_empty() {
			return None;
		}
		a[dim] = v;
	}

	return Some(a);
}

fn r1d_remove(include: &Range1D, exclude: &Range1D) -> [Range1D; 2] {
	[
		*include.start()..=*exclude.start()-1,
		*exclude.end()+1..=*include.end(),
	]
}

fn r3d_rem_dim(include: &mut Range3D, exclude: &Range3D, out: &mut [Option<Range3D>], dim: usize) {
	for (i, x) in r1d_remove(&include[dim], &exclude[dim]).iter().enumerate() {
		if x.is_empty() {
			continue;
		}
		let mut v = include.clone();
		v[dim] = x.clone();
		out[i] = Some(v);
	}
	include[dim] = exclude[dim].clone();
}

fn r3d_remove(include: &Range3D, exclude: &Range3D) -> [Option<Range3D>; 6] {
	if let Some(exclude) = r3d_union(include.clone(), exclude) {
		let mut include = include.clone();
		let mut out = [None, None, None, None, None, None];
		r3d_rem_dim(&mut include, &exclude, &mut out[0..], 0);
		r3d_rem_dim(&mut include, &exclude, &mut out[2..], 1);
		r3d_rem_dim(&mut include, &exclude, &mut out[4..], 2);
		out
	} else {
		// exclude does not include include
		[Some(include.clone()), None, None, None, None, None]
	}
}

fn r3d_volume(v: &Range3D) -> isize {
	v.iter().map(|x| (*x.end()+1) - *x.start()).reduce(|acc, v| acc * v).unwrap()
}

fn calc_on(stack: &[(bool, Range3D)], mask: &Range3D) -> isize {
	if stack.is_empty() {
		return 0;
	}
	let (on, range) = stack.last().unwrap();
	let mut sum = 0;
	if let Some(matching_range) = r3d_union(range.clone(), mask) {
		if *on {
			sum += r3d_volume(&matching_range);
		}
	}
	for bit in r3d_remove(mask, range).iter().filter_map(|x| x.clone()) {
		sum += calc_on(&stack[..stack.len() - 1], &bit);
	}
	sum
}

#[aoc(day22, part1)]
fn part1(input: &str) -> isize {
	solve(input, -50..=50)
}

#[aoc(day22, part2)]
fn part2(input: &str) -> isize {
	solve(input, std::isize::MIN..=std::isize::MAX)
}

fn solve(input: &str, range: Range1D) -> isize {
	let mask: Range3D = [range.clone(), range.clone(), range.clone()];
	let regex = regex::Regex::new(r"^(on|off) x=([0-9-]+)\.\.([0-9-]+),y=([0-9-]+)\.\.([0-9-]+),z=([0-9-]+)\.\.([0-9-]+)$").unwrap();
	let stack = input.lines().map(|line| {
		let r = regex.captures(line).expect(line);
		let is_on = r.get(1).unwrap().as_str() == "on";
		let mut vit = r.iter().skip(2).map(|x| x.unwrap().as_str().parse::<isize>().unwrap());
		(
			is_on,
			[
				vit.next().unwrap()..=vit.next().unwrap(),
				vit.next().unwrap()..=vit.next().unwrap(),
				vit.next().unwrap()..=vit.next().unwrap(),
			]
		)
	}).collect::<Vec<_>>();

	calc_on(stack.as_slice(), &mask)
}

#[cfg(test)]
const EXAMPLE_SM: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

#[test]
fn test_sm() {
	assert_eq!(part1(EXAMPLE_SM), 39);
}
