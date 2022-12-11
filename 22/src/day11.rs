use std::collections::VecDeque;
use crate::prelude::*;
use regex::Regex;
use ::num::integer::lcm;

#[aoc(day11, part1)]
fn day11_part1(input: &str) -> usize {
	let monkeys = parse(input);
	monkey_business(monkeys, 20, None)
}

#[aoc(day11, part2)]
fn day11_part2(input: &str) -> usize {
	let monkeys = parse(input);
	let lcm = monkeys.iter()
		.map(|x| x.test)
		.reduce(|a, b| lcm(a, b))
		.unwrap();
	monkey_business(monkeys, 10_000, Some(lcm))
}

#[derive(Debug)]
enum Operation {
	Add,
	Mul,
}
#[derive(Debug)]
enum Operand {
	Old,
	Constant(usize)
}
#[derive(Debug)]
struct Monkey {
	items: VecDeque<usize>,
	operation: Operation,
	operand: Operand,
	test: usize,
	if_true: usize,
	if_false: usize,
	num_inspected: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
	let op_re = Regex::new(r"Operation: new = old ([+*]) (old|[0-9]+)").unwrap();
	let trailing_num_re = Regex::new(r" ([0-9]+)$").unwrap();
	input.split("\n\n").map(|input| {
		let [_, sis, op, test, tr, fl] = input.lines().take_n::<6>().unwrap();
		let items = sis.split(": ").skip(1).next().unwrap().split(", ").must_parse::<usize>().collect();
		let [_, op, operand] = op_re.captures(op).unwrap().iter().take_n().unwrap();
		let operation = match op.unwrap().as_str() {
			"+" => Operation::Add,
			"*" => Operation::Mul,
			v => panic!("{}", v),
		};
		let operand = if "old" == operand.unwrap().as_str() {
			Operand::Old
		} else {
			Operand::Constant(operand.unwrap().as_str().parse().unwrap())
		};
		let test = trailing_num_re.captures(test).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
		let if_true = trailing_num_re.captures(tr).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
		let if_false = trailing_num_re.captures(fl).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();

		Monkey {
			items,
			operation,
			operand,
			test,
			if_true,
			if_false,
			num_inspected: 0,
		}
	})
	.collect()
}

fn monkey_business(mut monkeys: Vec<Monkey>, iterations: usize, modulo: Option<usize>) -> usize {
	for _round in 0..iterations {
		for monkey_no in 0..monkeys.len() {
			while let Some(item) = monkeys[monkey_no].items.pop_front() {
				monkeys[monkey_no].num_inspected += 1;
				let operand = match monkeys[monkey_no].operand {
					Operand::Old => item,
					Operand::Constant(v) => v,
				};
				let mut new_val = match monkeys[monkey_no].operation {
					Operation::Add => item + operand,
					Operation::Mul => item * operand,
				};
				if let Some(modulo) = modulo {
					new_val %= modulo;
				} else {
					new_val /= 3;
				}
				
				let target_monkey = if new_val % monkeys[monkey_no].test == 0 {
					monkeys[monkey_no].if_true
				} else {
					monkeys[monkey_no].if_false
				};

				assert_ne!(target_monkey, monkey_no);
				monkeys[target_monkey].items.push_back(new_val);
			}
		}
	}

	let mut num_inspections = monkeys.iter()
		.map(|x| x.num_inspected)
		.collect::<Vec<_>>();
	num_inspections.sort_by_key(|x| std::cmp::Reverse(*x));
	num_inspections[0] * num_inspections[1]
}