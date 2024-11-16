use crate::{Solution, Test};
use linkme::distributed_slice;

pub use linkme;

#[distributed_slice]
pub static SOLUTIONS: [Solution] = [..];

#[distributed_slice]
pub static TESTS: [Test] = [..];

#[distributed_slice]
pub static YEAR: [u32] = [..];

pub fn get_solution(day: Option<u8>, part: Option<u8>) -> Option<&'static Solution> {
	SOLUTIONS.iter()
		.filter(|s| {
			if let Some(d) = day {
				if d != s.day {
					return false;
				}
			}
			if let Some(p) = part {
				if p != s.part {
					return false;
				}
			}
			true
		})
		.max_by_key(|s| (s.day, s.part))
}
