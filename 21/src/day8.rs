#[aoc(day8, part1)]
fn day8_part1(input: &str) -> usize {
	let mut dist = 0;
	let interesting:[usize; 4] = [2, 4, 3, 7];
	for line in input.trim().lines() {
		let mut v = line.split("|").map(|x| x.trim().split(" "));
		let _input: Vec<_> = v.next().unwrap().collect();
		let output: Vec<_> = v.next().unwrap().collect();
		dist += output.iter().filter(|x| interesting.contains(&x.len())).count();
	}
	dist
}

#[aoc(day8, part2)]
fn day8_part2(input: &str) -> usize {
	const TOP: u8 = 0x01;
	const TL: u8 = 0x02;
	const TR: u8 = 0x04;
	const MID:u8 = 0x08;
	const BL:u8 = 0x10;
	const BR:u8 = 0x20;
	const BOT:u8 = 0x40;
	let display = [
		TOP | TL | TR | BL | BR | BOT, // 0
		TR | BR, // 1
		TOP | TR | MID | BL | BOT, // 2
		TOP | TR | MID | BR | BOT, // 3
		TL | TR | MID | BR, // 4
		TOP | TL | MID | BR | BOT, // 5
		TOP | TL | MID | BL | BR | BOT, // 6
		TOP | TR | BR, // 7
		TOP | TL | TR | MID | BL | BR | BOT, // 8
		TOP | TL | TR | MID | BR | BOT, // 9
	];
	input.trim().lines().map(|line| {
		let (input, output) = {
			let mut it = line.split("|")
				.map(|x| {
					x.trim().split(" ")
					.map(|part| part.as_bytes().iter()
						.map(|&c| 1 << (c - b'a'))
						.fold(0u8, |acc, x| acc | x))
					.collect::<Vec<_>>()
				});
			(it.next().unwrap(), it.next().unwrap())
		};
		// segment index -> scrambled display mask
		let mut mapping: [u8; 7] = [(1 << 7) - 1; 7];

		{
			let mut mask6s = (1 << 7) - 1;
			for &digit in input.iter() {
				if let Some(single) = match digit.count_ones() {
					2 => Some(1),
					3 => Some(7),
					4 => Some(4),
					7 => Some(8),
					6 => {
						mask6s &= digit;
						None
					},
					_ => None
				} {
					assert_eq!(digit.count_ones(), display[single].count_ones());
					for i in 0..7 {
						mapping[i] &= if (display[single] & (1 << i)) == 0 { !digit } else { digit };
					}
				}
			}

			for i in [0, 1, 5, 6] {
				mapping[i] &= mask6s;
			}
		}

		for i in 0..mapping.len() {
			if mapping[i].count_ones() != 1 {
				continue;
			}

			let first = 1 << mapping[i].trailing_zeros();
			for (ii, seg) in mapping.iter_mut().enumerate() {
				if i != ii {
					*seg &= !first;
				}
			}
		}

		if !mapping.iter().all(|x| x.count_ones() == 1) {
			panic!("{:X?}", mapping);
		}

		// scrambled index -> normal index
		let mapping_by_scrambled= {
			let mut out = [0u8; 7];
			for i in 0..7u8 {
				out[mapping[i as usize].trailing_zeros() as usize] = i;
			}
			out
		};

		output.iter().map(|segments| {
			let mask = (0..7)
				.filter(|x|segments & (1 << x) != 0)
				.map(|x| 1 << mapping_by_scrambled[x])
				.fold(0, |acc, v| acc | v);
			display.iter()
				.position(|&x| x == mask)
				.unwrap()
		}).fold(0, |acc, v| acc * 10 + v)
	}).sum::<usize>()
}

#[test]
fn test_part2() {
	assert_eq!(5353, day8_part2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"));
}