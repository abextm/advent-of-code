use regex::Regex;

fn advance_y(y: &mut isize, yvel: &mut isize) {
	*y += *yvel;
	*yvel -= 1;
}
fn advance_x(x: &mut isize, xvel: &mut isize) {
	*x += *xvel;
	*xvel -= xvel.signum();
}

fn parse(input: &str) -> ((isize, isize), (isize, isize)) {
	let cap = Regex::new(r"x=([-0-9]+)\.\.([-0-9]+), y=([-0-9]+)\.\.([-0-9]+)").unwrap()
		.captures(input).unwrap();
	let mut cap = cap.iter().skip(1)
		.map(|x| x.unwrap().as_str().parse::<isize>().expect(x.unwrap().as_str()));
	((cap.next().unwrap(), cap.next().unwrap()), (cap.next().unwrap(), cap.next().unwrap()))
}

#[aoc(day17, part1)]
fn part1(input: &str) -> isize {
	let (_, (ymin, ymax)) = parse(input);

	let mut o_ymax = 0;
	for mut yvel in 1..-ymin {
		let mut y = 0;
		let mut this_ymax = 0;
		let mut got_in = false;
		while y >= ymin {
			advance_y(&mut y, &mut yvel);
			this_ymax = std::cmp::max(this_ymax, y);
			got_in |= y >= ymin && y <= ymax;
		}
		if got_in {
			o_ymax = std::cmp::max(o_ymax, this_ymax);
		}
	}

	o_ymax
}

#[aoc(day17, part2)]
fn part2(input: &str) -> usize {
	let ((xmin, xmax), (ymin, ymax)) = parse(input);

	let mut ys = Vec::new();
	for orig_yvel in ymin..=-ymin {
		let mut yvel = orig_yvel;
		let mut y = 0;
		let mut got_in = false;
		while y >= ymin {
			advance_y(&mut y, &mut yvel);
			got_in |= y >= ymin && y <= ymax;
		}
		if got_in {
			ys.push(orig_yvel)
		}
	}

	let mut xs = Vec::new();
	for orig_xvel in 0..=xmax {
		let mut xvel = orig_xvel;
		let mut x = 0;
		let mut got_in = false;
		while x <= xmax && xvel > 0 {
			advance_x(&mut x, &mut xvel);
			got_in |= x <= xmax && x >= xmin
		}
		if got_in {
			xs.push(orig_xvel)
		}
	}

	ys.iter().flat_map(|&y| xs.iter().map(move |&x| (x, y)))
		.filter(|&(xvel_orig, yvel_orig)| {
			let mut xvel = xvel_orig;
			let mut yvel = yvel_orig;
			let mut y = 0;
			let mut x = 0;
			let mut got_in = false;
			while y >= ymin {
				advance_x(&mut x, &mut xvel);
				advance_y(&mut y, &mut yvel);
				got_in |= x <= xmax && x >= xmin && y >= ymin && y <= ymax;
			}
			got_in
		})
		.count()
}

#[test]
fn test() {
	assert_eq!(part2("target area: x=20..30, y=-10..-5"), 112);
}