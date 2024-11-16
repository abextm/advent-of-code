use std::cell::RefCell;
use std::collections::{hash_map, HashMap};
use std::convert::TryInto;
use std::fmt;
use std::rc::Rc;

// this is awful

#[derive(Clone)]
struct Node {
	rc: Rc<RefCell<InnerNode>>,
}

struct InnerNode {
	name: String,
	parent: Option<Node>,
	children: Vec<Node>,
}

struct NodeParentIter {
	current: Option<Node>,
}

impl Iterator for NodeParentIter {
	type Item = Node;
	fn next(&mut self) -> Option<Self::Item> {
		match self.current.clone() {
			Some(node) => {
				self.current = node.rc.borrow().parent.clone();
				self.current.clone()
			}
			None => None,
		}
	}
}

impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Node: {{{}}}", self.rc.borrow().name)
	}
}

impl PartialEq for Node {
	fn eq(&self, rhs: &Node) -> bool {
		Rc::ptr_eq(&self.rc, &rhs.rc)
	}
}

impl Node {
	fn get_parents(&self) -> NodeParentIter {
		NodeParentIter {
			current: Some(self.clone()),
		}
	}

	fn get_path_to(&self) -> Vec<Node> {
		let mut vec: Vec<Node> = self.get_parents().collect();
		vec.reverse();
		vec
	}

	fn set_parent(&self, parent: Node) {
		parent.rc.borrow_mut().children.push(self.clone());
		self.rc.borrow_mut().parent = Some(parent);
	}

	fn new(name: String) -> Node {
		Node {
			rc: Rc::new(RefCell::new(InnerNode {
				name: name,
				parent: None,
				children: Vec::new(),
			})),
		}
	}
}

fn get(s: String, map: &mut HashMap<String, Node>) -> &Node {
	match map.entry(s.clone()) {
		hash_map::Entry::Vacant(e) => e.insert(Node::new(s)),
		hash_map::Entry::Occupied(e) => e.into_mut(),
	}
}

#[aoc(part1=171213, part2=292)]
fn day6(input: &str, part2: bool) -> usize {
	let mut map = HashMap::<String, Node>::new();

	for line in input.trim().split("\n") {
		let slice: &[String] = &line
			.split(")")
			.take(2)
			.map(|x: &str| String::from(x))
			.collect::<Vec<_>>();
		let [parent, child]: &[String; 2] = slice.try_into().expect(line);
		let pnode = get(parent.clone(), &mut map).clone();
		get(child.clone(), &mut map).set_parent(pnode);
	}

	if !part2 {
		map.values().map(|x| x.get_parents().count()).sum()
	} else {
		let you_vec = map.get("YOU").expect("no you").get_path_to();
		let mut you = you_vec.iter();
		let san_vec = map.get("SAN").expect("no santa").get_path_to();
		let mut san = san_vec.iter();
		while you.next() == san.next() {}
		println!("!=");
		you.count() + san.count() + /* we ate already */2
	}
}
