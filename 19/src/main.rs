#![feature(iter_array_chunks)]

#[macro_use] extern crate aoc_helper;
#[macro_use] extern crate lazy_static;

pub mod vm;
pub mod taken;
pub mod itersort;

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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day17;
mod day18;
mod grid;

aoc_year!(19);

pub fn main() {
	aoc_helper::dispatch();
}