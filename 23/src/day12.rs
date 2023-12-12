use std::collections::HashMap;

const OPERATIONAL: u8 = b'.';
const DAMAGED: u8 = b'#';
const UNKNOWN: u8 = b'?';

#[derive(Copy, Clone, Debug)]
struct Range {
	value: u8,
	min: usize,
	rem_after: usize,
}

fn test(cache: &mut HashMap<(usize, usize), usize>, map: &[u8], ranges: &[Range]) -> usize {
	let r = ranges[0];
	if r.value == DAMAGED {
		if !map[0..(r.min)].iter().all(|&v| v == UNKNOWN || v == DAMAGED) {
			return 0;
		}

		return test(cache, &map[r.min..], &ranges[1..]);
	}

	assert_eq!(r.value, OPERATIONAL);

	if ranges.len() == 1 {
		return map.iter().all(|&v| v == UNKNOWN || v == OPERATIONAL) as usize;
	}

	if let Some(&v) = cache.get(&(map.len(), ranges.len())) {
		return v
	}

	let mut total_count = 0;

	for count in (r.min)..=(map.len() - r.rem_after) {
		if map[0..count].iter().all(|&v| v == UNKNOWN || v == OPERATIONAL) {
			total_count += test(cache, &map[count..], &ranges[1..]);
		}
	}

	cache.insert((map.len(), ranges.len()), total_count);

	total_count
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
	input.lines()
		.map(|line| {
			let [map, segments] = line.split(" ").next_chunk().unwrap();
			let segments = segments.split(",")
				.map(|x| x.parse::<usize>().unwrap())
				.collect::<Vec<_>>();
			let map = map.as_bytes();

			let mut ranges = Vec::new();
			ranges.push(Range{value: OPERATIONAL, min: 0, rem_after: 0});
			segments.iter()
				.map(|&v| Range{value: DAMAGED, min: v, rem_after: 0})
				.intersperse_with(|| Range{value: OPERATIONAL, min: 1, rem_after: 0})
				.for_each(|x| ranges.push(x));
			ranges.push(Range{value: OPERATIONAL, min: 0, rem_after: 0});

			let mut used = 0;
			ranges.iter_mut().rev()
				.for_each(|v| {
					v.rem_after = used;
					used += v.min;
				});

			test(&mut HashMap::new(), map, &ranges)
		})
		.sum()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
	input.lines()
		.map(|line| {
			let [in_map, segments] = line.split(" ").next_chunk().unwrap();
			let segments = segments.split(",")
				.map(|x| x.parse::<usize>().unwrap())
				.collect::<Vec<_>>()
				.repeat(5);

			let map = vec![in_map; 5].join("?");
			let map = map.as_bytes();

			let mut ranges = Vec::new();
			ranges.push(Range{value: OPERATIONAL, min: 0, rem_after: 0});
			segments.iter()
				.map(|&v| Range{value: DAMAGED, min: v, rem_after: 0})
				.intersperse_with(|| Range{value: OPERATIONAL, min: 1, rem_after: 0})
				.for_each(|x| ranges.push(x));
			ranges.push(Range{value: OPERATIONAL, min: 0, rem_after: 0});

			let mut used = 0;
			ranges.iter_mut().rev()
				.for_each(|v| {
					v.rem_after = used;
					used += v.min;
				});

			let v = test(&mut HashMap::new(), &map, &ranges);

			println!("{}", v);
			v
		})
		.sum()
}

#[test]
fn test_p2() {
	assert_eq!(525152, part2("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"));
}