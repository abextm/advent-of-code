use std::backtrace::Backtrace;
use std::cell::Cell;
use std::{fs, io};
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::panic::catch_unwind;
use std::time::Instant;
use clap::Parser;
use criterion::{Bencher, Criterion};
use reqwest::header::{HeaderMap, HeaderValue};

#[doc(hidden)]
pub mod internal;

pub use criterion;

pub use aoc_helper_macro::aoc;

#[derive(Parser)]
struct Args {
	#[arg(short, long)]
	day: Option<u8>,
	#[arg(short, long)]
	part: Option<u8>,
	#[arg(short, long)]
	test: Option<bool>,
	#[arg(long)]
	template: bool,
	#[arg(long)]
	bench: bool,
}

thread_local! {
	static BACKTRACE: Cell<Option<Backtrace>> = Cell::new(None);
	static TIME: Cell<Option<(Instant, Instant)>> = Cell::new(None);
}

fn get_year() -> u32 {
	if internal::YEAR.len() > 1 {
		panic!("multiple years defined");
	}

	let mut year = internal::YEAR[0];
	if year < 2000 {
		year += 2000;
	}
	year
}

pub fn dispatch() {
	let year = get_year();
	let args =  if let Ok(a) = std::env::var("AOC_ARGS") {
		Args::parse_from(["AOC_ARGS="].into_iter().chain(a.split(" ")))
	} else {
		Args::parse()
	};

	let solution = internal::get_solution(args.day, args.part);

	if args.template == true {
		let day = match solution {
			Some(v) => v.day + 1,
			None => 1,
		};

		template_solution(day);

		return;
	}

	let solution = match solution{
		Some(v) => v,
		None => panic!("No solutions"),
	};

	let input = get_input(year, solution.day);

	unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }

	if args.test != Some(false) {
		let old_hook = std::panic::take_hook();

		std::panic::set_hook(Box::new(|_| {
			BACKTRACE.set(Some(Backtrace::capture()));
		}));

		let mut num_all = 0;
		let mut num_ok = 0;

		for test in solution.tests() {
			println!("Test {}...", test.name);
			num_all += 1;
			let input = test.input.unwrap_or(&input);
			match catch_unwind(|| {solution.solve}(input)) {
				Ok(v) if v == test.result => {
					println!("Ok!");
					num_ok += 1;
				}
				Ok(v) => println!("Failed!\nGot\n\t{}\nWanted\n\t{}", v, test.result),
				Err(e) => {
					println!("Failed: {:?}", e);
					if let Some(b) = BACKTRACE.take() {
						println!("{:?}\n", b);
					}
				}
			}
		}

		std::panic::set_hook(old_hook);

		if num_all == num_ok {
			println!("All #[aoc] tests pass!")
		} else {
			println!("{} / {} tests passed. {} Failed!", num_ok, num_all, num_all - num_ok);
		}
	}

	if args.bench {
		let name = format!("{} day {} part {}", year, solution.day, solution.part);
		Criterion::default()
			.configure_from_args() // you have to use AOC_ARGS with --bench
			.profile_time(None) // bleh
			.bench_function(&name , |b| (solution.bench_fn)(b, &input));

		Criterion::default()
			.configure_from_args() // you have to use AOC_ARGS with --bench
			.final_summary();

		return;
	}

	if args.test != Some(true) {
		let start = Instant::now();
		TIME.set(Some((start, start)));
		let v = {solution.solve}(&input);
		time0("Done", false);
		TIME.set(None);
		println!("AoC {} day {} part {}: {}", year, solution.day, solution.part, v);
	}
}

pub fn time(message: &str) {
	time0(message, true);
}

fn time0(message: &str, force_double: bool) {
	if let Some(v) = TIME.get() {
		let now = Instant::now();
		if force_double || v.1 != v.0 {
			println!("{} {:?} ({:?} total)", message, now - v.1, now - v.0)
		} else {
			println!("Took {:?}", now - v.1);
		}
		TIME.set(Some((v.0, now)));
	}
}

pub struct Solution {
	pub day: u8,
	pub part: u8,
	pub solve: fn(&str) -> String,
	pub bench_fn: fn(&mut Bencher, &str) -> (),
}

pub struct Test {
	pub day: u8,
	pub part: u8,
	pub name: &'static str,
	pub input: Option<&'static str>,
	pub result: &'static str,
}

impl Solution {
	pub fn tests(&self) -> impl Iterator<Item=&'static Test> {
		let self_day = self.day;
		let self_part = self.part;
		internal::TESTS.iter()
			.filter(move |t| t.day == self_day && t.part == self_part)
	}
}

#[macro_export]
macro_rules! aoc_year {
  ($y: literal) => {
    #[::aoc_helper::internal::linkme::distributed_slice(::aoc_helper::internal::YEAR)]
		#[linkme(crate=::aoc_helper::internal::linkme)]
		static __abex_aoc_year: u32 = $y;
  };
}

impl Test {
	pub fn run(&self) {
		let solution = internal::get_solution(Some(self.day), Some(self.part))
			.unwrap();
		let v = if let Some(input) = self.input {
			(solution.solve)(&input)
		} else {
			let input = get_input(get_year(), self.day);
			(solution.solve)(&input)
		};

		assert_eq!(v, self.result);
	}
}

pub fn get_input(year: u32, day: u8) -> String {
	let path = format!("input/{day}.txt");
	match fs::read_to_string(&path) {
		Ok(v) => v,
		Err(e) if e.kind() == io::ErrorKind::NotFound => {
			let v = fetch_input(year, day);
			let _ = fs::create_dir("input");
			fs::write(path, &v).expect("unable to write input");
			v
		},
		Err(e) => panic!("error reading input{}", e),
	}
}

#[allow(deprecated)]
fn fetch_input(year: u32, day: u8) -> String {
	let mut path = std::env::home_dir().expect("no home dir");
	path.push(".config/abex-aoc-credentials.txt");
	match fs::read_to_string(&path) {
		Ok(token) if token != "" => {
			let mut headers = HeaderMap::new();
			headers.insert("User-Agent", HeaderValue::from_static("abex-aoc/1.0"));
			headers.insert("Cookie", HeaderValue::try_from(format!("session={}", token.trim())).unwrap());

			let client = reqwest::blocking::Client::builder()
				.default_headers(headers)
				.build()
				.unwrap();

			let mut req = client.get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
				.send()
				.expect("failed to download input");

			match req.status() {
				reqwest::StatusCode::OK => {
					let mut out = String::new();
					req.read_to_string(&mut out).unwrap();
					return out;
				},
				reqwest::StatusCode::NOT_FOUND => {
					panic!("puzzle not ready yet");
				},
				reqwest::StatusCode::BAD_REQUEST => {
					println!("invalid cookie");
				},
				code => {
					panic!("failed to download: {}", code)
				},
			}
		}
		Err(e) if e.kind() == io::ErrorKind::NotFound => {
			let _ = fs::write(path, "");
		}
		Err(e) => {
			panic!("failed to read token: {}", e);
		}
		Ok(_) => {},
	}

	panic!("Please fill ~/.config/abex-aoc-credentials.txt with session token (cookie)");
}

fn template_solution(day: u8) {
	OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(format!("src/day{}.rs", day))
		.expect("failed to open template file")
		.write_all("#[aoc()]
fn solve(input: &str) -> impl std::fmt::Debug {


	0
}".as_ref()).expect("Failed to write template");

	let mut main_fi = OpenOptions::new()
		.write(true)
		.read(true)
		.open("src/main.rs")
		.expect("failed to open main.rs");

	let mut main = String::new();
	main_fi.read_to_string(&mut main).unwrap();

	let index = match main.rfind("mod day") {
		Some(index) => index + main[index..].find("\n").unwrap_or(main.len() - index),
		None => main.len(),
	};

	main_fi.seek(io::SeekFrom::Start(index as u64)).unwrap();
	main_fi.write_all(format!("\nmod day{};", day).as_ref()).unwrap();
	main_fi.write_all(main[index..].as_ref()).unwrap();

}