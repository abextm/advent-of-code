use z3::ast::Ast;

fn part1_0(input: &str, range_min: i64, range_max: i64) -> usize {
	let hail = input.trim().lines()
		.map(|line| {
			let [pos, delta] = line.split(" @ ")
				.map(|bit| bit.split(", ").map(|x| x.parse::<i64>().expect(x)).array_chunks::<2>().next().unwrap())
				.array_chunks::<2>().next().unwrap();
			(pos, delta)
		}).collect::<Vec<_>>();

	let range = range_min..=range_max;

	hail.iter().enumerate()
		.map(|(a_i, (a_pos, a_delta))| {
			hail.iter().enumerate().skip(a_i).filter(|(b_i, (b_pos, b_delta))| {
				if a_i == *b_i {
					return false;
				}
				let x1 = a_pos[0] as f64;
				let x2 = (a_pos[0] + a_delta[0]) as f64;
				let x3 = b_pos[0] as f64;
				let x4 = (b_pos[0] + b_delta[0]) as f64;
				let y1 = a_pos[1] as f64;
				let y2 = (a_pos[1] + a_delta[1]) as f64;
				let y3 = b_pos[1] as f64;
				let y4 = (b_pos[1] + b_delta[1]) as f64;

				let ua = ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
				let ub = ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));

				let intersection_x = x1 + (ua * (x2-x1));
				let intersection_y = y1 + (ua * (y2-y1));

				if ua < 0.0 || ub < 0.0 {
					return false;
				}

				range.contains(&(intersection_x as i64))
					&& range.contains(&(intersection_y as i64))
			}).count()
		}).sum()
}

#[aoc(day24, part1)]
fn part1(input: &str) -> usize {
	part1_0(input, 200_000_000_000_000, 400_000_000_000_000)
}

#[test]
fn test_p1() {
	assert_eq!(2, part1_0("19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3", 7, 27));
}

const AXIS: [&str; 3] = ["x", "y", "z"];

#[aoc(day24, part2)]
fn part2(input: &str) -> i64 {
	let ctx = z3::Context::new(&z3::Config::new());
	let solver = z3::Solver::new(&ctx);

	let start = AXIS.map(|c| z3::ast::Real::new_const(&ctx, format!("{}_start", c)));
	let rate = AXIS.map(|c| z3::ast::Real::new_const(&ctx, format!("{}_rate", c)));

	input.trim().lines()
		.for_each(|line| {
			let [c_start, c_rate] = line.split(" @ ")
				.map(|bit| bit.split(", ").map(|x| {
					let v = x.parse::<i64>().expect(x);
					z3::ast::Int::from_i64(&ctx, v).to_real()
				}).array_chunks::<3>().next().unwrap())
				.array_chunks::<2>().next().unwrap();

			let intersect_time = z3::ast::Real::fresh_const(&ctx, "intersect_time");

			for x in 0..3 {
				solver.assert(&((&start[x] + (&intersect_time * &rate[x]))._eq(&(&c_start[x] + (&intersect_time * &c_rate[x])))));
			}
		});

	assert_eq!(solver.check(), z3::SatResult::Sat);
	let model = solver.get_model().unwrap();
	start.iter().map(|var| {
		let r = model.eval(var, true).unwrap().as_real().unwrap();
		assert_eq!(r.1, 1);
		r.0
	}).sum()
}