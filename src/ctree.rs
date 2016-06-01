use ctok::*;
use ns::*;
use super::*;

pub fn to_tree<T: Iterator<Item=char>>(t: Tokenizer<T>, ns: &mut Namespace) -> Node {
	let mut tree = Group(ns.to_int("document"), Vec::new());
	for tok in t {
		match tree {
			Group(_, ref mut children) => {
				children.push(match tok {
					Token::STRING(s) => Group(ns.to_int("string"), vec![Atom(ns.to_int(&s))]),
					Token::OPER(c) => Group(ns.to_int("oper"), vec![Atom(ns.to_int(&c.to_string()))]),
					Token::NUM(s) => Group(ns.to_int("num"), vec![Atom(ns.to_int(&s))]),
					Token::IDENT(s) => Group(ns.to_int("ident"), vec![Atom(ns.to_int(&s))]),
					_ => unreachable!(),
				});
			},
			_ => unreachable!(),
		}
	}
	tree
}
