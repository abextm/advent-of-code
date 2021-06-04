#[aoc(day18, part1)]
fn day18_part1(input: &str) -> i64 {
	input.trim().split("\n").map(|x| {
		let tokens = tokenize(x);
		parse(&mut tokens.iter().peekable())
	}).sum()
}

fn parse<'a>(input: &mut std::iter::Peekable<impl Iterator<Item=&'a Token>>) -> i64 {
	let mut v = parse_num(input);
	loop {
		let op = match input.next() {
			Some(&Token::Plus) => Token::Plus,
			Some(&Token::Mul) => Token::Mul,
			Some(&Token::RParen) => return v,
			None => return v,
			v => panic!("{:?}", v),
		};
		let rhs = parse_num(input);
		v = match op {
			Token::Plus => v + rhs,
			Token::Mul => v * rhs,
			_ => unreachable!(),
		};
	}
}

fn parse_num<'a>(input: &mut std::iter::Peekable<impl Iterator<Item=&'a Token>>) -> i64 {
	match input.next() {
		Some(&Token::Number(v)) => v,
		Some(&Token::LParen) => parse(input),
		v => panic!("{:?}", v),
	}
}

#[test]
fn p1() {
	assert_eq!(day18_part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
}

#[aoc(day18, part2)]
fn day18_part2(input: &str) -> i64 {
	input.trim().split("\n").map(|x| {
		let tokens = tokenize(x);
		parse_prec(&mut tokens.iter().peekable())
	}).sum()
}

fn parse_prec<'a>(input: &mut std::iter::Peekable<impl Iterator<Item=&'a Token>>) -> i64 {
	let mut shunt = std::collections::VecDeque::new();
	let mut out = Vec::new();
	loop {
		let tok = input.next();
		match tok {
			Some(&Token::Number(v)) => out.push(Token::Number(v)),
			Some(&Token::LParen) => out.push(Token::Number(parse_prec(input))),
			Some(&Token::RParen) => break,
			Some(&Token::Plus) | Some(&Token::Mul) => {
				let tok = *tok.unwrap();
				while let Some(v) = shunt.pop_back() {
					if v == Token::Plus {
						out.push(v);
					} else {
						shunt.push_back(v);
						break;
					}
				}
				shunt.push_back(tok);
			}
			None => break,
		};
	}
	while let Some(v) = shunt.pop_back() {
		out.push(v);
	}
	let mut nums = std::collections::VecDeque::new();
	for tok in out {
		match tok {
			Token::Number(v) => nums.push_back(v),
			Token::Plus => {let v = nums.pop_back().unwrap() + nums.pop_back().unwrap(); nums.push_back(v)},
			Token::Mul => {let v = nums.pop_back().unwrap() * nums.pop_back().unwrap(); nums.push_back(v)},
			v => panic!("{:?}", v),
		}
	}
	nums.pop_back().unwrap()
}


#[test]
fn p2() {
	assert_eq!(day18_part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Token {
	Number(i64),
	LParen,
	RParen,
	Plus,
	Mul,
}

fn tokenize(input: &str) -> Vec<Token> {
	let mut input = input.chars();
	let mut out = Vec::new();
	loop {
		loop {
			out.push(match input.next() {
				Some(' ') => continue,
				Some('(') => Token::LParen,
				Some(')') => Token::RParen,
				Some('+') => Token::Plus,
				Some('*') => Token::Mul,
				Some(v) if v >= '0' && v <= '9' => Token::Number(v as i64 - '0' as i64),
				Some(v) => panic!("{}", v),
				None => return out,
			});
		}
	}
}