#[aoc(day22, part1)]
fn day22_part1(input: &str) -> usize {
	let mut players = input
		.trim()
		.split("\n\n")
		.map(|i| {
			i.split("\n")
				.skip(1)
				.map(|v| v.parse::<usize>().unwrap())
				.collect::<std::collections::VecDeque<_>>()
		})
		.collect::<Vec<_>>();

	let mut min_id = 0;
	while players.iter().all(|x| x.len() != 0) {
		let mut top = players.iter_mut().enumerate().map(|(i, a)| (i, a.pop_front().unwrap())).collect::<Vec<_>>();
		top.sort_by_key(|(i, a)| -(*a as i64));
		min_id = top[0].0;
		for (_, v) in top {
			players[min_id].push_back(v);
		}
	}

	let player = &players[min_id];
	player.iter().enumerate().map(|(i, &v)| (player.len() - i)*v).sum()
}

#[aoc(day22, part2)]
fn day22_part2(input: &str) -> usize {
	let mut players = input
		.trim()
		.split("\n\n")
		.map(|i| {
			i.split("\n")
				.skip(1)
				.map(|v| v.parse::<usize>().unwrap())
				.collect::<std::collections::VecDeque<_>>()
		})
		.collect::<Vec<_>>();

	let mut game = 0;
	recurse(players, &mut game).1
}

fn recurse(mut players: Vec<std::collections::VecDeque<usize>>, id: &mut usize) -> (usize, usize) {
	let rid = *id;
	*id+=1;
	let mut min_id = 0;
	let mut prev_games = std::collections::HashSet::new();
	while players.iter().all(|x| x.len() != 0) {
		//println!("{} {:?}", rid, players);
		if !prev_games.insert(players.clone()) {
			min_id = 0;
			break;
		}
		let mut top = players.iter_mut().enumerate().map(|(i, a)| (i, a.pop_front().unwrap())).collect::<Vec<_>>();
		if top.iter().all(|&(id, val)| players[id].len() >= val) {
			(min_id, _) = recurse(players.iter().zip(top.iter()).map(|(p, (_, val))| {
				let mut out = std::collections::VecDeque::new();
				for i in 0..*val {
					out.push_back(p[i]);
				}
				out
			}).collect(), id);//?
			players[min_id].push_back(top[min_id].1);
			players[min_id].push_back(top[min_id^1].1);
		} else {
			top.sort_by_key(|(_, a)| -(*a as i64));
			min_id = top[0].0;
			for (_, v) in top {
				players[min_id].push_back(v);
			}
		}
	}


	println!("{} {} won", rid, min_id);
	let player = &players[min_id];
	(min_id, player.iter().enumerate().map(|(i, &v)| (player.len() - i)*v).sum())
}