use crate::grid::Grid;

const OPERATIONAL: u8 = b'.';
const DAMAGED: u8 = b'#';
const UNKNOWN: u8 = b'?';

#[derive(Copy, Clone, Debug)]
struct Range {
	value: u8,
	min: usize,
	rem_after: usize,
	index: usize,
}

fn test(cache: &mut Grid<Vec<usize>>, map: &[u8], ranges: &[Range]) -> usize {
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

	let v = cache[[map.len(), r.index]];
	if v > 0 {
		return v - 1;
	}

	let mut total_count = 0;

	for count in (r.min)..=(map.len() - r.rem_after) {
		if map[0..count].iter().all(|&v| v == UNKNOWN || v == OPERATIONAL) {
			total_count += test(cache, &map[count..], &ranges[1..]);
		}
	}

	cache[[map.len(), r.index]] = total_count + 1;

	total_count
}

fn solve(map: &[u8], segments: &[usize]) -> usize {
	let mut ranges = Vec::new();
	ranges.push(Range{value: OPERATIONAL, min: 0, rem_after: 0, index: 0});
	segments.iter()
		.map(|&v| Range{value: DAMAGED, min: v, rem_after: 0, index: 0})
		.intersperse_with(|| Range{value: OPERATIONAL, min: 1, rem_after: 0, index: 0})
		.for_each(|x| ranges.push(x));
	ranges.push(Range{value: OPERATIONAL, min: 0, rem_after: 0, index: 0});

	let mut used = 0;
	let mut index = 0;
	ranges.iter_mut().rev()
		.for_each(|v| {
			v.rem_after = used;
			used += v.min;
			if v.value == OPERATIONAL {
				v.index = index;
				index += 1;
			}
		});

	test(&mut Grid::blank(&(map.len() + 1, index), 0usize), map, &ranges)
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
	input.lines()
		.map(|line| {
			let [map, segments] = line.split(" ").next_chunk().unwrap();
			let segments = segments.split(",")
				.map(|x| x.parse::<usize>().unwrap())
				.collect::<Vec<_>>();
			
			solve(map.as_bytes(), &segments)
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
			
			solve(map.as_bytes(), &segments)
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