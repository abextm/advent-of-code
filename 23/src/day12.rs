const OPERATIONAL: u8 = b'.';
const DAMAGED: u8 = b'#';
const UNKNOWN: u8 = b'?';

#[derive(Copy, Clone, Debug)]
struct Range {
	value: u8,
	min: usize,
	rem_after: usize,
}

fn test(map: &[u8], ranges: &[Range]) -> usize {
	println!("{}", map.len());

	let r = ranges[0];
	if r.value == DAMAGED {
		if !map[0..(r.min)].iter().all(|&v| v == UNKNOWN || v == DAMAGED) {
			return 0;
		}

		return test(&map[r.min..], &ranges[1..]);
	}

	assert_eq!(r.value, OPERATIONAL);

	if ranges.len() == 1 {
		return map.iter().all(|&v| v == UNKNOWN || v == OPERATIONAL) as usize;
	}

	let mut total_count = 0;

	for count in (r.min)..=(map.len() - r.rem_after) {
		if map[0..count].iter().all(|&v| v == UNKNOWN || v == OPERATIONAL) {
			total_count += test(&map[count..], &ranges[1..]);
		}
	}

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

			test(map, &ranges)
		})
		.sum()
}

#[test]
fn test_p1() {
	assert_eq!(21, part1("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"));
}