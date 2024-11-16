#[aoc(part1=3342351, part2=5010664)]
fn solve(input: &str, part: usize) -> i32 {
	input
		.trim()
		.split("\n")
		.map(|str| {
			let val = str.parse::<i32>().expect("not an int");
			let mut fuel = fuel_for_mass(val);
			if part == 2 {
				let mut new_weight = fuel;
				while new_weight > 0 {
					new_weight = fuel_for_mass(new_weight);
					fuel += new_weight;
				}
			}
			fuel
		})
		.sum::<i32>()
}

fn fuel_for_mass(mass: i32) -> i32 {
	let fuel = (mass / 3) - 2;
	if fuel < 0 {
		0
	} else {
		fuel
	}
}
