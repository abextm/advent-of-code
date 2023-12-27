use std::collections::HashMap;

fn count_nodes(visited: &mut [bool], skip: &[(usize, usize); 3], connections: &[Vec<usize>], node: usize) -> usize {
	if visited[node] {
		return 0;
	}
	visited[node] = true;

	let skip_me = skip.iter()
		.filter_map(|&(a, b)| if a == node { Some(b) } else if b == node { Some(a) } else { None })
		.next()
		.unwrap_or(usize::MAX);

	connections[node].iter()
		.filter(|&&edge| edge != skip_me)
		.map(|&edge| count_nodes(visited, skip, connections, edge))
		.sum::<usize>() + 1
}

#[aoc(day25, part1)]
fn part1(input: &str) -> usize {
	let mut connections = Vec::<Vec<usize>>::new();
	let mut cid_by_name = HashMap::<&str, usize>::new();
	let mut edges = Vec::new();

	for line in input.lines() {
		let (key, rem) = line.split_once(": ").unwrap();
		let key = *cid_by_name.entry(key).or_insert_with(|| {
			connections.push(Vec::new());
			connections.len() - 1
		});
		for other in rem.split(" ") {
			let other = *cid_by_name.entry(other).or_insert_with(|| {
				connections.push(Vec::new());
				connections.len() - 1
			});

			//println!("{} -> {};", key, other);
			connections[key].push(other);
			connections[other].push(key);
			edges.push((key, other));
		}
	}

	let mut visited = vec![false; edges.len()];

	assert_eq!(connections.len(), count_nodes(&mut visited, &[(usize::MAX, usize::MAX); 3], &connections, 0));
	visited.fill(false);

	let skip_edges = [(1422, 1286), (514, 691), (380, 379)];
	let size = count_nodes(&mut visited, &skip_edges, &connections, skip_edges[0].0);
	visited.fill(false);
	let size2 = count_nodes(&mut visited, &skip_edges, &connections, skip_edges[0].1);
	visited.fill(false);
	println!("{}", edges.len());
	println!("{} {} {}", size, size2, size * size2);
	if size != connections.len() {
		return size * (connections.len() - size);
	}
	visited.fill(false);

	panic!();
}