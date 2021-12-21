#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
	let mut players = input.lines().map(|line|{ 
		(line.split(": ").skip(1).next().unwrap().parse::<usize>().unwrap() - 1, 0)
	}).collect::<Vec<(usize, usize)>>();
	let mut die = 0;


	let win_pid = 
'outer:
	loop {
		for (pid, player) in players.iter_mut().enumerate() {
			for _ in 0..3 {
				die += 1;
				player.0 += die % 100;
			}
			player.0 %= 10;
			player.1 += player.0 + 1;
			if player.1 >= 1000 {
				break 'outer pid;
			}
		}
	};
	players[win_pid^1].1 * die
}

#[derive(Clone, Copy, Debug)]
struct PlayerState {
	score: usize,
	position: usize,
}

fn pack_states(states: [PlayerState; 2]) -> usize {
	let [a, b] = states.map(|s| (s.position * 21) + s.score);
	a * 210 + b
}

#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
	let dices = {
		let mut d = [0usize; 10];
		for a in 1..=3 {
			for b in 1..=3 {
				for c in 1..=3 {
					d[a + b + c] += 1;
				}
			}
		}
		d.into_iter().enumerate()
			.filter(|(_, count)| *count > 0)
			.collect::<Vec<_>>()
	};
	assert_eq!(dices.iter().map(|(_, v)| *v).sum::<usize>(), 27);

	let mut wins = [0usize; 2];
	let mut states = [0usize; 210 * 210];
	let known_states = (0..states.len()).map(|state| {
		let a = state / 210;
		let b = state % 210;
		let s = [
			PlayerState{
				score: a % 21,
				position: a / 21,
			},
			PlayerState{
				score: b % 21,
				position: b / 21,
			},
		];
		assert_eq!(pack_states(s), state);
		s
	}).collect::<Vec<_>>();

	{
		let mut si = input.lines().map(|line| {
			let start_pos = line.split(": ").skip(1).next().unwrap().parse::<usize>().unwrap();
			PlayerState {
				score: 0,
				position: start_pos - 1,
			}
		});
		states[pack_states([si.next().unwrap(), si.next().unwrap()])] += 1;
	}
	
	let mut states2 = states.clone();
	let mut from = &mut states;
	let mut to = &mut states2;
	for _ in 0.. {
		let mut cont = 0;
		for i in 0..2 {
			to.fill(0);
			for state in 0..from.len() {
				let in_count = from[state];
				if in_count <= 0 {
					continue;
				}
				let s = &known_states[state];
				'states:
				for &(bval, bcount) in &dices {
					let new_count = bcount * in_count;
					let mut ns = *s;
					ns[i].position = (ns[i].position + bval) % 10;
					ns[i].score += ns[i].position + 1;
					if ns[i].score >= 21 {
						wins[i] += new_count;
						continue 'states;
					}
					to[pack_states(ns)] += new_count;
					cont += 1;
				}
			}

			std::mem::swap(&mut from, &mut to);
		}

		if cont == 0 {
			break
		}
	}

	*wins.iter().max().unwrap()
}

#[cfg(test)]
const EXAMPLE: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

#[test]
fn test() {
	assert_eq!(part2(EXAMPLE), 444356092776315);
}