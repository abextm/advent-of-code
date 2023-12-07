use std::cmp::Reverse;

#[derive(Debug)]
struct Hand {
	cards: [u8; 5],
	bid: usize,
	typ: HandType,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
enum HandType {
	Kind5,
	Kind4,
	FullHouse,
	Kind3,
	Pair2,
	Pair1,
	Distinct,
}

impl Hand {
	fn parse(s: &str, part2: bool) -> Hand {
		let mut hand = Hand {
			cards: [0; 5],
			bid: s[6..].parse().unwrap(),
			typ: HandType::Distinct,
		};

		let mut jokers = 0;
		let mut duplicates = Vec::<(u8, u8)>::new();

		for i in 0..5 {
			let code =  match s.as_bytes()[i] {
				v if (b'0'..=b'9').contains(&v) => v - b'0',
				b'T' => 10,
				b'J' => if part2 { 0 } else { 11 },
				b'Q' => 12,
				b'K' => 13,
				b'A' => 14,
				v => panic!("{}", v),
			};
			hand.cards[i] = code;

			if code == 0 {
				jokers += 1;
			} else {
				if let Some(dup) = duplicates.iter_mut().find(|(id, _count)| code == *id) {
					dup.1 += 1;
				} else {
					duplicates.push((code, 1));
				}
			}
		}

		duplicates.sort_by_key(|&(_id, count)| Reverse(count));

		if duplicates.is_empty() {
			duplicates.push((0, 0));
		}
		duplicates[0].1 += jokers;

		hand.typ = match (duplicates[0].1, duplicates.len()) {
			(5, 1) => HandType::Kind5,
			(4, 2) => HandType::Kind4,
			(3, 2) => HandType::FullHouse,
			(3, 3) => HandType::Kind3,
			(2, 3) => HandType::Pair2,
			(2, 4) => HandType::Pair1,
			(1, 5) => HandType::Distinct,
			v => panic!("{:?} {:?}", v, duplicates),
		};

		hand
	}
}

fn solve(input: &str, part2: bool) -> usize {
	let mut hands = input.lines()
		.map(|x| Hand::parse(x, part2))
		.collect::<Vec<_>>();

	hands.sort_by_key(|h| Reverse((h.typ, Reverse(h.cards))));

	hands.iter()
		.enumerate()
		.map(|(index, hand)| hand.bid * (index + 1))
		.sum()
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
	solve(input, false)
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
	solve(input, true)
}


#[cfg(test)]
const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[test]
fn test_p1() {
	assert_eq!(6440, part1(EXAMPLE))
}

#[test]
fn test_p2() {
	assert_eq!(5905, part2(EXAMPLE))
}