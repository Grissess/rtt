use std::collections::HashMap;
use std::collections::hash_map::Entry;

use super::*;

pub struct Namespace {
	int_str: HashMap<usize, String>,
	str_int: HashMap<String, usize>,
	next_int: usize,
	bad_value: String
}

impl Namespace {
	pub fn new() -> Namespace {
		Namespace {
			int_str: HashMap::new(),
			str_int: HashMap::new(),
			next_int: 0,
			bad_value: "BAD_VALUE".to_string(),
		}
	}

	pub fn to_int(&mut self, key: &str) -> usize {
		match self.str_int.entry(key.to_string()) {
			Entry::Occupied(oe) => *oe.get(),
			Entry::Vacant(ve) => {
				let result = self.next_int;
				self.next_int += 1;
				ve.insert(result);
				self.int_str.insert(result, key.to_string());
				result
			}
		}
	}

	pub fn to_str(&self, key: usize) -> Option<&String> {
		self.int_str.get(&key)
	}

	pub fn print(&self, node: &Node) {
		match *node {
			Atom(val) => print!("{:?}", self.to_str(val).unwrap_or(&self.bad_value)),
			Group(val, ref children) => {
				print!("{}", self.to_str(val).unwrap_or(&self.bad_value));
				self.print_children(children);
			}
			MatchPoint(val) => print!("<{}>", val),
			Sequence(val, ref children) => {
				print!("<{}>", self.to_str(val).unwrap_or(&self.bad_value));
				self.print_children(children);
			},
			Conjunctor(ref children) => {
				print!("&");
				self.print_children(children);
			},
			Disjunctor(ref children) => {
				print!("|");
				self.print_children(children);
			},
			Negator(ref child) => {
				print!("!");
				self.print(child);
			},
			SplicePair(start, len) => {
				print!("SPLICE_PAIR(START:{},LEN:{})", start, len);
			},
			NoNode => print!("NO_NODE"),
		}
	}

	fn print_children(&self, children: &Vec<Node>) {
		print!("[");
		for child in children.iter().take(children.len() - 1) {
			self.print(child);
			print!(", ");
		}
		if let Some(child) = children.last() {
			self.print(child);
		}
		print!("]");
	}

	pub fn debug_print(&self, node: &Node) {
		self.debug_print_over(node, 0);
	}

	fn debug_print_over(&self, node: &Node, indent: usize) {
		print!("{}", (0..indent).map(|_| "  ").collect::<String>());
		match *node {
			Atom(val) => println!("Atom: {:?}", self.to_str(val)),
			Group(val, ref children) => {
				println!("Group: {:?}", self.to_str(val));
				for child in children {
					self.debug_print_over(child, indent + 1);
				}
			},
			MatchPoint(val) => println!("MatchPoint: {:?}", self.to_str(val)),
			Sequence(val, ref children) => {
				println!("Sequence: {:?}", self.to_str(val));
				for child in children {
					self.debug_print_over(child, indent + 1);
				}
			},
			Conjunctor(ref children) => {
				println!("Conjunctor:");
				for child in children {
					self.debug_print_over(child, indent + 1);
				}
			},
			Disjunctor(ref children) => {
				println!("Disjunctor:");
				for child in children {
					self.debug_print_over(child, indent + 1);
				}
			},
			Negator(ref inner) => {
				println!("Negator:");
				self.debug_print_over(inner, indent + 1);
			},
			SplicePair(start, len) => println!("SplicePair: start {} len {}", start, len),
			NoNode => println!("NoNode"),
		}
	}
}
