#![feature(iter_array_chunks)]

#[macro_use] extern crate aoc_runner_derive;
extern crate aoc_runner;

mod iter_utils;

#[allow(dead_code)]
mod grid;

mod prelude {
	pub use crate::iter_utils::{TakeN, IterFromStr};
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;

aoc_lib! {year=2022}
