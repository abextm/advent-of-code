use tungstenite::Message;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::net::TcpListener;
use crate::vm;
use crate::vm::{EvalError, EvalResult};

#[aoc()]
fn solve(input: &str) -> impl std::fmt::Debug {
	let server = TcpListener::bind("127.0.0.1:8000").unwrap();
	let vm = vm::new_from_str(input);
	for stream in server.incoming() {
		let vm = vm.clone();
		std::thread::spawn(move || {
			let mut ws = tungstenite::accept(stream.unwrap()).unwrap();
			let input = RefCell::new(VecDeque::<u8>::new());
			let in2 = &input;
			let mut vm = vm.unwrap().with_input(std::iter::from_fn(|| in2.borrow_mut().pop_front().map(|x| x as i64)));

			loop {
				let mut output = String::new();
				loop {
					match vm.single_step() {
						Ok(EvalResult::Output(v)) => output.push(v as u8 as char),
						Ok(EvalResult::Ok) => continue,
						Err(EvalError::EndOfInput) => {
							println!("{}", &output);
							ws.write(Message::Text(output)).unwrap();
							break;
						},
						Ok(EvalResult::Return) => {
							println!("{}", &output);
							ws.write(Message::Text(output)).unwrap();
							ws.close(None).unwrap();
							return;
						}
						v => panic!("{:?}", v),
					}
				}
				loop {
					ws.flush().unwrap();
					match ws.read() {
						Ok(Message::Text(v)) => {
							in2.borrow_mut().extend(v.bytes());
							break;
						}
						Ok(Message::Ping(v)) => ws.write(Message::Pong(v)).unwrap(),
						Ok(Message::Close(_)) => (),
						Err(tungstenite::Error::ConnectionClosed) => return,
						v => panic!("{:?}", v),
					}
				}
			}
		});
	}
}