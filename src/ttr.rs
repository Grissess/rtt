use super::*;
use ns::*;

#[allow(non_snake_case)]
pub fn make_ttr_rules(ns: &mut Namespace) -> RuleSet {
	let mut rules = RuleSet::new();

	// Namespace initialization
	let _string = ns.to_int("string");
	let _oper = ns.to_int("oper");
	let _ident = ns.to_int("ident");

	let _Atom = ns.to_int("Atom");
	let _MatchPoint = ns.to_int("MatchPoint");
	let _Group = ns.to_int("Group");
	let _Sequence = ns.to_int("Sequence");
	let _Child = ns.to_int("Child");
	let _Children = ns.to_int("Children");
	let _Conjunctor = ns.to_int("Conjunctor");
	let _Disjunctor = ns.to_int("Disjunctor");
	let _Negator = ns.to_int("Negator");
	let _Rule = ns.to_int("Rule");
	let _Rules = ns.to_int("Rules");
	let _RuleSet = ns.to_int("RuleSet");

	let _sequence = ns.to_int("sequence");
	let _x = ns.to_int("x");
	let _y = ns.to_int("y");
	let _a = ns.to_int("a");
	let _b = ns.to_int("b");

	let __lang = ns.to_int("<");
	let __rang = ns.to_int(">");
	let __lbra = ns.to_int("[");
	let __rbra = ns.to_int("]");
	let __lpar = ns.to_int("(");
	let __rpar = ns.to_int(")");
	let __exclm = ns.to_int("!");
	let __comma = ns.to_int(",");
	let __dash = ns.to_int("-");
	let __scol = ns.to_int(";");
	let __bar = ns.to_int("|");
	let __amp = ns.to_int("&");
	let __empty = ns.to_int("");

	// Atoms and MatchPoints
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_string, vec![MatchPoint(_x)])]),
		rhs: Sequence(_sequence, vec![Group(_Atom, vec![MatchPoint(_x)])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_oper, vec![Atom(__lang)]), Group(_ident, vec![MatchPoint(_x)]), Group(_oper, vec![Atom(__rang)])]),
		rhs: Sequence(_sequence, vec![Group(_MatchPoint, vec![MatchPoint(_x)])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_oper, vec![Atom(__lang)]), Group(_oper, vec![Atom(__rang)])]),
		rhs: Sequence(_sequence, vec![Group(_MatchPoint, vec![Atom(__empty)])]),
	});

	// Groups and Sequences
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_Atom, vec![MatchPoint(_x)]), Group(_Children, vec![MatchPoint(_y)])]),
		rhs: Sequence(_sequence, vec![Group(_Group, vec![MatchPoint(_x), Group(_Children, vec![MatchPoint(_y)])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_ident, vec![MatchPoint(_x)]), Group(_Children, vec![MatchPoint(_y)])]),
		rhs: Sequence(_sequence, vec![Group(_Group, vec![MatchPoint(_x), Group(_Children, vec![MatchPoint(_y)])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_MatchPoint, vec![MatchPoint(_x)]), Group(_Children, vec![MatchPoint(_y)])]),
		rhs: Sequence(_sequence, vec![Group(_Sequence, vec![MatchPoint(_x), Group(_Children, vec![MatchPoint(_y)])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_oper, vec![Atom(__lpar)]), Group(_ident, vec![MatchPoint(_x)]), Group(_oper, vec![Atom(__rpar)]), Group(_Children, vec![MatchPoint(_y)])]),
		rhs: Sequence(_sequence, vec![Group(_Sequence, vec![MatchPoint(_x), Group(_Children, vec![MatchPoint(_y)])])]),
	});

	// Disjunctors and Conjunctors
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_oper, vec![Atom(__bar)]), Group(_Children, vec![MatchPoint(_x)])]),
		rhs: Sequence(_sequence, vec![Group(_Disjunctor, vec![Group(_Children, vec![MatchPoint(_x)])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_oper, vec![Atom(__amp)]), Group(_Children, vec![MatchPoint(_x)])]),
		rhs: Sequence(_sequence, vec![Group(_Conjunctor, vec![Group(_Children, vec![MatchPoint(_x)])])]),
	});

	// TTR Group Templates with Arity
	let templates = vec![
		Group(_Atom, vec![MatchPoint(_x)]),
		Group(_MatchPoint, vec![MatchPoint(_x)]),
		Group(_Group, vec![MatchPoint(_x), MatchPoint(_y)]),
		Group(_Sequence, vec![MatchPoint(_x), MatchPoint(_y)]),
		Group(_Disjunctor, vec![MatchPoint(_x)]),
		Group(_Conjunctor, vec![MatchPoint(_y)]),
	];

	// Negators
	for template in &templates {
		rules.push(Rule {
			lhs: Sequence(_sequence, vec![Group(_oper, vec![Atom(__exclm)]), template.clone()]),
			rhs: Sequence(_sequence, vec![Group(_Negator, vec![template.clone()])]),
		});
	}

	// Children
	// - Initiators
	for template in &templates {
		rules.push(Rule {
			lhs: Sequence(_sequence, vec![Group(_oper, vec![Atom(__lbra)]), template.clone()]),
			rhs: Sequence(_sequence, vec![Group(_Child, vec![template.clone()])]),
		});
	}
	// - Continuations, arity 1
	for template in &templates {
		rules.push(Rule {
			lhs: Sequence(_sequence, vec![Group(_Child, vec![MatchPoint(_a)]), Group(_oper, vec![Atom(__comma)]), template.clone()]),
			rhs: Sequence(_sequence, vec![Group(_Child, vec![MatchPoint(_a), template.clone()])]),
		});
	}
	// - Continuations, arity 2
	for template in &templates {
		rules.push(Rule {
			lhs: Sequence(_sequence, vec![Group(_Child, vec![MatchPoint(_a), MatchPoint(_b)]), Group(_oper, vec![Atom(__comma)]), template.clone()]),
			rhs: Sequence(_sequence, vec![Group(_Child, vec![Group(_Child, vec![MatchPoint(_a), MatchPoint(_b)]), template.clone()])]),
		});
	}
	// - Terminators
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_Child, vec![MatchPoint(_a)]), Group(_oper, vec![Atom(__rbra)])]),
		rhs: Sequence(_sequence, vec![Group(_Children, vec![Group(_Child, vec![MatchPoint(_a)])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_Child, vec![MatchPoint(_a), MatchPoint(_b)]), Group(_oper, vec![Atom(__rbra)])]),
		rhs: Sequence(_sequence, vec![Group(_Children, vec![Group(_Child, vec![MatchPoint(_a), MatchPoint(_b)])])]),
	});

	// Rules
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_Atom, vec![MatchPoint(_x)]), Group(_oper, vec![Atom(__dash)]), Group(_oper, vec![Atom(__rang)]), Group(_Atom, vec![MatchPoint(_a)])]),
		rhs: Sequence(_sequence, vec![Group(_Rule, vec![Group(_Atom, vec![MatchPoint(_x)]), Group(_Atom, vec![MatchPoint(_a)])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_Atom, vec![MatchPoint(_x)]), Group(_oper, vec![Atom(__dash)]), Group(_oper, vec![Atom(__rang)]), Group(_Group, vec![MatchPoint(_a), MatchPoint(_b)])]),
		rhs: Sequence(_sequence, vec![Group(_Rule, vec![Group(_Atom, vec![MatchPoint(_x)]), Group(_Group, vec![MatchPoint(_a), MatchPoint(_b)])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_Group, vec![MatchPoint(_x), MatchPoint(_y)]), Group(_oper, vec![Atom(__dash)]), Group(_oper, vec![Atom(__rang)]), Group(_Atom, vec![MatchPoint(_a)])]),
		rhs: Sequence(_sequence, vec![Group(_Rule, vec![Group(_Group, vec![MatchPoint(_x), MatchPoint(_y)]), Group(_Atom, vec![MatchPoint(_a)])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_Group, vec![MatchPoint(_x), MatchPoint(_y)]), Group(_oper, vec![Atom(__dash)]), Group(_oper, vec![Atom(__rang)]), Group(_Group, vec![MatchPoint(_a), MatchPoint(_b)])]),
		rhs: Sequence(_sequence, vec![Group(_Rule, vec![Group(_Group, vec![MatchPoint(_x), MatchPoint(_y)]), Group(_Group, vec![MatchPoint(_a), MatchPoint(_b)])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_Sequence, vec![MatchPoint(_x), MatchPoint(_y)]), Group(_oper, vec![Atom(__dash)]), Group(_oper, vec![Atom(__rang)]), Group(_Sequence, vec![MatchPoint(_a), MatchPoint(_b)])]),
		rhs: Sequence(_sequence, vec![Group(_Rule, vec![Group(_Sequence, vec![MatchPoint(_x), MatchPoint(_y)]), Group(_Sequence, vec![MatchPoint(_a), MatchPoint(_b)])])]),
	});

	// Ruleset
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_Rule, vec![MatchPoint(_x), MatchPoint(_y)]), Group(_oper, vec![Atom(__scol)])]),
		rhs: Sequence(_sequence, vec![Group(_RuleSet, vec![Group(_Rules, vec![Group(_Rule, vec![MatchPoint(_x), MatchPoint(_y)])])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_RuleSet, vec![Group(_Rules, vec![MatchPoint(_a)])]), Group(_Rule, vec![MatchPoint(_x), MatchPoint(_y)]), Group(_oper, vec![Atom(__scol)])]),
		rhs: Sequence(_sequence, vec![Group(_RuleSet, vec![Group(_Rules, vec![MatchPoint(_a), Group(_Rule, vec![MatchPoint(_x), MatchPoint(_y)])])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_RuleSet, vec![Group(_Rules, vec![MatchPoint(_a), MatchPoint(_b)])]), Group(_Rule, vec![MatchPoint(_x), MatchPoint(_y)]), Group(_oper, vec![Atom(__scol)])]),
		rhs: Sequence(_sequence, vec![Group(_RuleSet, vec![Group(_Rules, vec![Group(_Rules, vec![MatchPoint(_a), MatchPoint(_b)]), Group(_Rule, vec![MatchPoint(_x), MatchPoint(_y)])])])]),
	});
	rules.push(Rule {
		lhs: Sequence(_sequence, vec![Group(_RuleSet, vec![MatchPoint(_x)]), Group(_RuleSet, vec![MatchPoint(_y)])]),
		rhs: Sequence(_sequence, vec![Group(_RuleSet, vec![Group(_Rules, vec![MatchPoint(_x), MatchPoint(_y)])])]),
	});
	rules
}

#[cfg(test)]
mod tests {
	use super::*;
	use ns::*;

	#[test]
	fn disp_ttr_rules() {
		let mut ns = Namespace::new();
		println!("{:?}", make_ttr_rules(&mut ns));
	}
}
