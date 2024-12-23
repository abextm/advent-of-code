use std::collections::{HashMap, HashSet};
use itertools::Itertools;

type Key = [u8; 2];

#[aoc(part1 = 1476)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let mut links = HashMap::<Key, Vec<Key>>::new();
	for line in input.trim().lines() {
		let [a, b] = line.split("-").map(|v| v.bytes().next_chunk().unwrap()).next_chunk().unwrap();
		let al = links.entry(a).or_default();
		if !al.contains(&b) {
			al.push(b);
			links.entry(b).or_default().push(a);
		}
	}

	let mut skip = HashSet::new();

	let links = &links;
	links.iter()
		.flat_map(|(k, v)| {
			v.iter().combinations(2).filter_map(move |v| {
				let [a, b] = v.try_into().unwrap();
				if !links.get(a).unwrap().contains(b) {
					return None;
				}
				let mut v = [k, a, b];
				v.sort();
				Some(v)
			})
		})
		.filter(|v| v.iter().any(|k| k[0] == b't'))
		.filter(|n| skip.insert(*n))
		.inspect(|v| println!("{:?}", v.map(|aaa| std::str::from_utf8(aaa).unwrap())))
		.count()
}

#[aoc(part2 = "\"ca,dw,fo,if,ji,kg,ks,oe,ov,sb,ud,vr,xr\"")]
fn part2(input: &str) -> impl std::fmt::Debug {
	let mut links = HashMap::<Key, Vec<Key>>::new();
	for line in input.trim().lines() {
		let [a, b] = line.split("-").map(|v| v.bytes().next_chunk().unwrap()).next_chunk().unwrap();
		let al = links.entry(a).or_insert_with(|| vec![a]);
		if !al.contains(&b) {
			al.push(b);
			links.entry(b).or_insert_with(|| vec![b]).push(a);
		}
	}

	let mut best = Vec::new();

	for (k, v) in links.iter() {
		let proj = v.iter()
			.filter(|node| { links.get(*node).unwrap().contains(k) })
			.collect::<Vec<_>>();

		for rem_num in 0..proj.len() {
			if proj.len() - rem_num < best.len() {
				break;
			}
			for rems in (1..proj.len()).combinations(rem_num) {
				let inner = proj.iter().cloned().enumerate().filter(|(i, _v)| !rems.contains(i)).map(|(_i, v)| v).collect::<Vec<_>>();
				if inner.iter().all(|node| {
					let nn = links.get(*node).unwrap();
					inner.iter().all(|inode| nn.contains(inode))
				}) {
					best = inner;
				}
			}
		}
	}

	best.sort();
	best.iter().map(|n| std::str::from_utf8(*n).unwrap()).collect::<Vec<_>>().join(",")
}


#[aoc(part1 = 7, part2 = "\"co,de,ka,ta\"")]
const EX: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";