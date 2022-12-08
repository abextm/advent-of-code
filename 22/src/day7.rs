use std::collections::HashMap;

fn parse(input: &str) -> HashMap<String, usize> {
	let mut path: String = "/".into();
	let mut out = HashMap::<String, usize>::new();

	let mut it = input.lines().peekable();
	while let Some(sh) = it.next() {
		let mut bits = sh.split(" ").skip(1);
		let cmd = bits.next().unwrap();
		if cmd == "cd" {
			let mut rel_path = bits.next().unwrap();
			if rel_path.starts_with("/") {
				path = "/".into();
				rel_path = &rel_path[1..];
			}
			if rel_path.len() > 0 {
				for bit in rel_path.split("/") {
					if bit == ".." {
						path.truncate(path.rfind("/").expect("too much .."));
					} else {
						path.push_str(bit);
						path.push_str("/");
					}
				}
			}
		} else if cmd == "ls" {
			let mut dir_size = 0usize;
			while let Some(line) = it.next_if(|&x| !x.starts_with("$")) {
				let size = line.split(" ").next().unwrap();
				if size != "dir" {
					dir_size += size.parse::<usize>().unwrap();
				}
			}

			let mut d = &path[..];
			loop {
				*out.entry(d.into())
					.or_insert(0)
					+= dir_size;
				
				if d == "/" {
					break;
				}
				let sl = d[..d.len() - 1].rfind("/").unwrap() + 1;
				d = &d[..sl];
			}
		} else {
			panic!("cmd: {}", cmd);
		}
	}

	out
}

#[aoc(day7, part1)]
fn day7_part1(input: &str) -> usize {
	parse(input).values()
		.filter(|&&x| x <= 100000)
		.sum()
}

#[aoc(day7, part2)]
fn day7_part2(input: &str) -> usize {
	let input = parse(input);
	let total_size = *input.get("/").unwrap();
	let min_free = total_size - (70000000 - 30000000);
	*input.values()
		.filter(|&&x| x >= min_free)
		.min()
		.unwrap()
}