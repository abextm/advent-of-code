struct RangeMap {
	src: usize,
	dst: usize,
	len: usize,
}

impl RangeMap {
	fn src_end(&self) -> usize {
		self.src + self.len
	}
}

struct RangeMapSet {
	src_type: String,
	dst_type: String,
	ranges: Vec<RangeMap>
}

impl RangeMapSet {
	fn find(&self, id: usize) -> usize {
		for range in &self.ranges {
			if id >= range.src && id < range.src + range.len {
				return id - range.src + range.dst
			}
		}
		id
	}

	fn parse(input: &str) -> Vec<RangeMapSet> {
		regex::Regex::new(r"([a-z]+)-to-([a-z]+) map:\n([0-9 \n]+)").unwrap()
		.captures_iter(input)
		.map(|cap| {
			RangeMapSet {
				src_type: cap[1].into(),
				dst_type: cap[2].into(),
				ranges: cap[3].trim().lines().map(|line| {
					let mut line = line.split(" ").map(|p| p.parse::<usize>().unwrap());
					RangeMap {
						dst: line.next().unwrap(),
						src: line.next().unwrap(),
						len: line.next().unwrap(),
					}
				}).collect(),
			}
		})
		.collect()
	}
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
	let ranges = RangeMapSet::parse(input);
	regex::Regex::new(r"seeds: ([0-9 ]+)")
		.unwrap()
		.captures(input)
		.unwrap()[1]
		.split(" ")
		.map(|x| x.parse::<usize>().unwrap())
		.map(|seed| {
			let mut id = seed;
			for ms in &ranges {
				id = ms.find(id)
			}
			id
		})
		.min()
		.unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
	let ranges = RangeMapSet::parse(input);
	regex::Regex::new(r"seeds: ([0-9 ]+)")
		.unwrap()
		.captures(input)
		.unwrap()[1]
		.split(" ")
		.map(|x| x.parse::<usize>().unwrap())
		.array_chunks::<2>()
		.map(|[seed_start, seed_len]| {
			let mut in_range = vec![[seed_start, seed_start + seed_len]];
			let mut out_range = Vec::<[usize; 2]>::new();
			let mut extra = Vec::<[usize; 2]>::new();
			for ms in &ranges {
				for mr in &ms.ranges {
					for r in &in_range {
						if r[0] > mr.src_end() || mr.src > r[1] {
							extra.push(*r)
						} else {
							let intersect = [mr.src.max(r[0]), mr.src_end().min(r[1])];
							if r[0] != intersect[0] {
								extra.push([r[0], intersect[0]])
							}
							if r[1] != intersect[1] {
								extra.push([intersect[1], r[1]])
							}
							out_range.push([intersect[0] - mr.src + mr.dst, intersect[1] - mr.src + mr.dst]);
						}
					}
					[in_range, extra] = [extra, in_range];
					extra.clear();
				}
				out_range.extend(&in_range);

				[out_range, in_range] = [in_range, out_range];
				out_range.clear();
			}
			in_range.iter()
				.map(|x| x[0])
				.min()
				.unwrap()
		})
		.min()
		.unwrap()
}