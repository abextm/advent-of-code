
#[aoc(day3, part1)]
fn day3_part1(input: &str) -> i32 {
	let input = input.trim();
	let n = input.find('\n').expect("nn");
	let mut v0 = vec![0; n];
	let mut v1 = vec![0; n];
	for str in input.split("\n") {
		for i in 0..str.len() {
			if str.as_bytes()[i] == '0' as u8 {
				v0[i]+=1;
			} else {
				v1[i]+=1;
			}
		}
	}

	let mut o = 0;
	for i in 0..n {
		o <<= 1;
		o |= if v1[i] > v0[i] { 1 } else {0};
	}
	o * (o ^ ((1<<n) - 1))
}


#[aoc(day3, part2)]
fn day3_part2(input: &str) -> i32 {
	let input = input.trim();
	let rating: Vec<_> = input.split("\n").collect();
	filt(rating.clone(), false) * filt(rating, true)
}

fn filt(mut rating: Vec<&str>, inv: bool) -> i32 {
	for bit in 0.. {
		let mut vs = (0, 0);
		for line in rating.iter() {
			if line.as_bytes()[bit] == '0' as u8 {
				vs.0+=1;
			} else {
				vs.1+=1;
			}
		}
		let t = if (vs.0 > vs.1) ^ inv {'0'} else {'1'} as u8;
		rating = rating.into_iter()
			.filter(|line| line.as_bytes()[bit] == t)
			.collect();
		
		if rating.len() <= 1 {
			break;
		}
	}

	let mut o = 0;
	for c in rating[0].as_bytes() {
		o <<= 1;
		if *c == '1' as u8 {
			o |= 1;
		}
	}
	o
}