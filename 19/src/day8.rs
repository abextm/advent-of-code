#[aoc(day8, part1)]
fn day8_part1(input: &str) -> usize {
	checksum(decode(input, (25, 6)))
}

fn checksum(layers: Vec<Vec<u8>>) -> usize {
	layers
		.iter()
		.min_by_key(|x| x.iter().filter(|x| **x == 0).count())
		.unwrap()
		.iter()
		.filter(|x| **x == 1)
		.count()
		* layers[0].iter().filter(|x| **x == 2).count()
}

fn decode(input: &str, dims: (usize, usize)) -> Vec<Vec<u8>> {
	let layer_len = dims.0 * dims.1;
	let mut iter = input.chars().map(|x| x as u8 - '0' as u8);
	(0..)
		.map(|_| iter.by_ref().take(layer_len).collect::<Vec<u8>>())
		.take_while(|x| x.len() != 0)
		.collect::<Vec<Vec<u8>>>()
}

#[test]
fn checksum_example() {
	assert_eq!(checksum(decode("123456789012", (3, 2))), 1);
}

#[aoc(day8, part2)]
fn day8_part2(input: &str) -> String {
	let dims = (25, 6);
	let img = composite(decode(input, dims));
	draw_layer(&img, dims)
}

mod colors {
	pub const BLACK: u8 = 0;
	pub const WHITE: u8 = 1;
	pub const TRANS: u8 = 2;
}

fn composite(img: Vec<Vec<u8>>) -> Vec<u8> {
	let mut iter = img.into_iter().rev();
	let mut out = iter.next().expect("Must have atleast 1 layer");
	for layer in iter {
		for i in 0..out.len() {
			let px = layer[i];
			if px != colors::TRANS {
				out[i] = px;
			}
		}
	}
	out
}

#[test]
fn composite_example() {
	assert_eq!(
		composite(decode("0222112222120000", (2, 2))),
		decode("0110", (2, 2))[0]
	);
}

const COLORS: [char; 3] = ['█', '░', ' '];

fn draw_layer(layer: &Vec<u8>, dims: (usize, usize)) -> String {
	let mut out = String::with_capacity(1 + (dims.0 + 1) * dims.1);
	out.push('\n');
	let mut iter = layer.iter().map(|x| COLORS[*x as usize]);
	for _y in 0..(dims.1) {
		for _x in 0..(dims.0) {
			out.push(iter.next().unwrap());
		}
		out.push('\n');
	}
	out
}
