#![feature(io)]

use std::io;
use std::io::Read;

extern crate rtt;

use rtt::ctok::*;
use rtt::ctree::*;
use rtt::ns::*;
use rtt::ttr::*;
use rtt::*;

fn main() {
	let mut ns = Namespace::new();
	let rules = make_ttr_rules(&mut ns);
	let mut tree = to_tree(Tokenizer::new(io::stdin().chars().map(|r| r.unwrap())), &mut ns);
	let result = run(&tree, &rules);
	tree = result.0;
	println!("{} iters:", result.1);
	ns.print(&tree);
}
