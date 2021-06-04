#[derive(Copy, Clone, Debug)]
struct Node<T> {
	next: usize,
	id: usize,
	value: T,
}

struct Input<T> {
	items: Vec<Node<T>>,
}

impl<T> Input<T> {
	fn iter<'a>(&'a self, start: usize) -> InputIter<'a, T> {
		InputIter {
			input: self,
			current: start,
		}
	}
}

struct InputIter<'a, T> {
	input: &'a Input<T>,
	current: usize,
}

impl<'a, T> Iterator for InputIter<'a, T> {
	type Item = &'a Node<T>;
	fn next(&mut self) -> Option<&'a Node<T>> {
		let n = &self.input.items[self.current];
		self.current = n.next;
		Some(n)
	}
}

#[aoc(day23, part1)]
fn day23_part1(input: &str) -> String {
	let mut cups = Input {
		items: input
			.trim()
			.chars()
			.enumerate()
			.map(|(i, c)| Node {
				next: i + 1,
				id: i,
				value: (c as u8 - '1' as u8) as usize,
			})
			.collect(),
	};
	let mut max = cups.items.len();
	cups.items.iter_mut().last().unwrap().next = 0;
	let mut current = 0;
	for _ in 0..100 {
		let i = cups.items[current].next;
		let badend = *cups.iter(i).skip(2).next().unwrap();
		cups.items[current].next = badend.next;
		let bad = cups
			.iter(i)
			.take(3)
			.map(|n| n.value)
			.collect::<Vec<_>>();
		let mut val = (cups.items[current].value + max - 1) % max;
		while bad.iter().find(|v| **v == val).is_some() {
			val = (val + max - 1) % max;
		}

		let dst = cups.iter(current).find(|n| n.value == val).unwrap().id;
		let end = cups.items[dst].next;
		cups.items[dst].next = i;
		cups.items[badend.id].next = end;
		current = cups.items[current].next;
	}
	String::from_utf8(
		cups
			.iter(0)
			.skip_while(|n| n.value != 0)
			.take(max)
			.skip(1)
			.map(|n| n.value as u8 + '1' as u8)
			.collect(),
	)
	.unwrap()
}

#[test]
fn testp1() {
	assert_eq!(day23_part1("389125467"), "67384529");
}

#[aoc(day23, part2)]
fn day23_part2(input: &str) -> usize {
	let mut cups = Input {
		items: input
			.trim()
			.chars()
			.map(|c| (c as u8 - '1' as u8) as usize)
			.chain(input.len()..1_000_000)
			.enumerate()
			.map(|(i, v)| Node {
				next: i + 1,
				id: i,
				value: v,
			})
			.collect(),
	};
	let mut node_by_value = vec![0; cups.items.len()];
	for node in cups.items.iter() {
		node_by_value[node.value] = node.id;
	}
	let mut max = cups.items.len();
	cups.items.iter_mut().last().unwrap().next = 0;
	let mut current = 0;
	for i in 0..10_000_000 {
		let i = cups.items[current].next;
		let badend = *cups.iter(i).skip(2).next().unwrap();
		cups.items[current].next = badend.next;
		let bad = cups
			.iter(i)
			.take(3)
			.map(|n| n.value)
			.collect::<Vec<_>>();
		let mut val = (cups.items[current].value + max - 1) % max;
		while bad.iter().find(|v| **v == val).is_some() {
			val = (val + max - 1) % max;
		}

		let dst = node_by_value[val];
		let end = cups.items[dst].next;
		cups.items[dst].next = i;
		cups.items[badend.id].next = end;
		current = cups.items[current].next;
	}
	cups
		.iter(node_by_value[0])
		.skip(1)
		.take(2)
		.map(|n| n.value + 1)
		.product()
}

/*#[test]
fn testp2() {
	assert_eq!(day23_part2("389125467"), 149245887792);
}*/
