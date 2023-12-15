fn hash(str: &str) -> u8 {
	str.as_bytes().iter().fold(0, |acc, v| v.wrapping_add(acc).wrapping_mul(17))
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
	input.trim().split(",")
		.map(|str| hash(str) as usize)
		.sum::<usize>()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
	let pat = regex::Regex::new("(.*)(-|=([0-9]))").unwrap();

	let mut boxes = vec![Vec::<(&str, u8)>::new(); 256];
	input.trim().split(",")
		.for_each(|str| {
			let cap = pat.captures(str).unwrap();
			let label = &str[cap.get(1).unwrap().range()];
			let box_index = hash(label) as usize;
			let box0 = &mut boxes[box_index];
			match cap.get(3) {
				None => {
					box0.retain(|&(e_label, _)| e_label != label)
				},
				Some(focal_len) => {
					let focal_len = focal_len.as_str().parse().unwrap();
					match box0.iter_mut().find(|(e_label, _)| *e_label == label) {
						Some(b) => { b.1 = focal_len; },
						None => {
							box0.push((label, focal_len));
						}
					}
				},
			};
		});

	boxes.iter()
		.enumerate()
		.flat_map(|(boxi, box0)| box0.iter()
			.enumerate()
			.map(move |(i, &(_, len))| {
				let v = (boxi + 1) as usize * (i + 1) * len as usize;
				v
			}))
			.sum::<usize>()
}