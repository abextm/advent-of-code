#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]

#[macro_use] extern crate aoc_helper;

mod grid;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

aoc_year!(24);

pub fn main() {
	aoc_helper::dispatch();
}