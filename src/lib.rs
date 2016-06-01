use std::boxed::Box;
use std::iter;
use std::sync::RwLock;

extern crate hamt;
use hamt::HamtMap;

pub mod ctok;
pub mod ctree;
pub mod ns;
pub mod ttr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
	// Ordinary nodes
	Atom(usize),
	Group(usize, Vec<Node>),
	// Pattern nodes
	MatchPoint(usize),
	Sequence(usize, Vec<Node>),
	Conjunctor(Vec<Node>),
	Disjunctor(Vec<Node>),
	Negator(Box<Node>),
	// Dirty hacks
	SplicePair(usize, usize),
	NoNode,
}

pub use Node::*;

pub type Bindings = HamtMap<usize, RwLock<Node>>;

impl Node {
	pub fn is_ordinary(&self) -> bool {
		match *self {
			Atom(..) => true,
			MatchPoint(..) | Sequence(..) | Conjunctor(..) | Disjunctor(..) | Negator(..) | SplicePair(..) | NoNode => false,
			Group(_, ref v) => {
				v.iter().all(Node::is_ordinary)
			}
		}
	}

	pub fn matches(&self, other: &Node, bindings: Bindings) -> (bool, Bindings) {
		match *self {
			Atom(lval) => {
				if let Atom(rval) = *other {
					(lval == rval, bindings)
				} else { (false, bindings) }
			},
			Group(lname, ref lvec) => {
				if let Group(rname, ref rvec) = *other {
					if lname != rname { return (false, bindings); }
					let mut ret_bindings = bindings.clone();
					if lvec.len() != rvec.len() { return (false, bindings); }
					for (lref, rref) in lvec.iter().zip(rvec) {
						let result = lref.matches(rref, ret_bindings);
						if !result.0 {
							return (false, bindings);
						}
						ret_bindings = result.1;
					}
					(true, ret_bindings)
				} else { (false, bindings) }
			},
			MatchPoint(idx) => {
				match bindings.clone().find(&idx) {
					Some(noderef) => noderef.read().unwrap().matches(other, bindings),
					None => (true, bindings.plus(idx, RwLock::new(other.clone()))),
				}
			},
			Sequence(idx, ref lvec) => {
				if let Group(_, ref rvec) = *other {
					let (llen, rlen) = (lvec.len(), rvec.len());
					if llen > rlen { return (false, bindings); }
					let limit = rlen - llen + 1;
					for i in 0..limit {
						let mut ret_bindings = bindings.clone();
						let mut succeeded = true;
						for (lref, rref) in lvec.iter().zip(rvec.iter().skip(i)) {
							let result = lref.matches(rref, ret_bindings.clone());
							if !result.0 {
								succeeded = false;
								break;
							}
							ret_bindings = result.1;
						}
						if succeeded {
							return (true, ret_bindings.plus(idx, RwLock::new(SplicePair(i, llen))));
						}
					}
					(false, bindings)
				} else { (false, bindings) }
			},
			Conjunctor(ref lvec) => {
				let mut ret_bindings = bindings.clone();
				for lref in lvec {
					let result = lref.matches(other, ret_bindings);
					if !result.0 { return (false, bindings); }
					ret_bindings = result.1;
				}
				(true, ret_bindings)
			},
			Disjunctor(ref lvec) => {
				for lref in lvec {
					let result = lref.matches(other, bindings.clone());
					if result.0 { return result; }
				}
				(false, bindings)
			},
			Negator(ref lref) => {
				(!(*lref).matches(other, bindings.clone()).0, bindings)
			},
			_ => panic!("Not a pattern tree element: {:?}", self),
		}
	}

	pub fn eval(&self, other: &Node, bindings: &Bindings) -> Node {
		match *self {
			MatchPoint(idx) => bindings.find(&idx).expect("Could not find matched binding").read().unwrap().clone(),
			Group(name, ref lvec) => {
				let empty_v = Vec::new();
				let (iterable, ilen) = if let Group(_, ref rvec) = *other {
					(rvec.iter().cloned(), rvec.len())
				} else {
					(empty_v.iter().cloned(), 0)
				};
				let llen = lvec.len();
				let mut children = Vec::with_capacity(llen);
				children.extend(lvec.iter().cloned().zip(iterable.chain(iter::repeat(NoNode).take(if llen > ilen { llen - ilen } else { 0 }))).map(|(lref, rref)| {
					lref.eval(&rref, bindings)
				}));
				Group(name, children)
			},
			Sequence(idx, ref lvec) => {
				if let Group(name, ref rvec) = *other {
					let (sidx, slen) = if let Some(ref rwsref) = bindings.find(&idx) {
						if let SplicePair(sidx, slen) = *(rwsref.read().unwrap()) {
							(sidx, slen)
						} else { panic!("Seq identifier refers to a non-splice binding") }
					} else { panic!("Seq identifier not found in bindings") };
					let (llen, rlen) = (lvec.len(), rvec.len());
					let mut children = Vec::with_capacity(rlen + llen - slen);
					children.extend(
						rvec.iter().cloned().take(sidx).chain(
							lvec.iter().cloned().zip(rvec.iter().cloned().chain(iter::repeat(NoNode)).skip(sidx).take(llen)).map(|(lref, rref)| {
								lref.eval(&rref, bindings)
							}).chain(
								rvec.iter().cloned().skip(sidx + slen)
							)
						)
					);
					Group(name, children)
				} else { panic!("Can't extrapolate Seq to non-Group") }
			},
			_ => self.clone(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Rule {
	pub lhs: Node,
	pub rhs: Node,
}

pub type RuleSet = Vec<Rule>;

impl Rule {
	pub fn exec(&self, tree: &Node) -> (bool, Node) {
		let mut bindings = Bindings::new();
		let result = self.lhs.matches(tree, bindings);
		if !result.0 { return (false, NoNode); }
		bindings = result.1;
		(true, self.rhs.eval(tree, &bindings))
	}
}

pub fn pass(tree: Node, rules: &RuleSet) -> (bool, Node) {
	for rule in rules {
		let (changed, node) = rule.exec(&tree);
		if changed {
			return (changed, node)
		}
	}
	(false, NoNode)
}

pub fn run(tree: &Node, rules: &RuleSet) -> (Node, u32) {
	let mut iters = 0u32;
	let mut mtree = tree.clone();
	let mut changed = true;
	while changed {
		let result = pass(mtree.clone(), rules);
		changed = result.0;
		if changed {
			mtree = result.1;
			iters += 1;
		}
	}
	(mtree, iters)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn simple_seq() {
		let mut tree = Group(1, vec![Atom(1), Atom(3), Atom(2), Atom(3), Atom(3), Atom(1)]);
		let rules = vec![Rule { lhs: Sequence(1, vec![Atom(3)]), rhs: Sequence(1, vec![Atom(4), Atom(5)]) }];
		println!("{:?}", tree);
		let result = super::run(&tree, &rules);
		tree = result.0;
		println!("{:?}", tree);
		println!("In {} iterations", result.1);
		assert_eq!(tree, Group(1, vec![Atom(1), Atom(4), Atom(5), Atom(2), Atom(4), Atom(5), Atom(4), Atom(5), Atom(1)]));
	}

	#[test]
	fn simple_seq_group() {
		let mut tree = Group(1, vec![Atom(2), Group(2, vec![Atom(1), Atom(5), Group(3, vec![])]), Atom(1), Group(2, vec![Atom(1)]), Atom(3)]);
		let rules = vec![Rule { lhs: Sequence(1, vec![Group(2, vec![Atom(1)])]), rhs: Sequence(1, vec![Group(3, vec![Group(4, vec![Atom(1)])])]) }];
		println!("{:?}", tree);
		let result = super::run(&tree, &rules);
		tree = result.0;
		println!("{:?}", tree);
		println!("In {} iterations", result.1);
		assert_eq!(tree, Group(1, vec![Atom(2), Group(2, vec![Atom(1), Atom(5), Group(3, vec![])]), Atom(1), Group(3, vec![Group(4, vec![Atom(1)])]), Atom(3)]));
	}
}
