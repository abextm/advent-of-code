#![feature(iter_array_chunks)]

#[macro_use] extern crate aoc_runner_derive;
extern crate aoc_runner;

mod iter_utils;

mod prelude {
	pub use crate::iter_utils::{TakeN, IterFromStr};
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

aoc_lib! {year=2022}
