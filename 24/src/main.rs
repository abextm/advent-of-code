#![feature(iter_next_chunk)]

#[macro_use] extern crate aoc_helper;

mod grid;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

aoc_year!(24);

pub fn main() {
	aoc_helper::dispatch();
}