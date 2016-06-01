use std::io::Read;
use std::collections::HashMap;
use std::iter::Iterator;

#[allow(non_snake_case)]
pub struct Lexemes<'a> {
	WS: &'a str,
	PUNCT: &'a str,
	STR_GRP: &'a str,
	ESCAPE: char,
	ESCAPES: HashMap<char, char>,
	ESC_HEX: char,
	ESC_OCT: char,
	OCT_DIGIT: &'a str,
	DIGIT: &'a str,
	HEX_DIGIT: &'a str,
	IDENT_START: &'a str,
	IDENT: &'a str,
}

impl<'a> Lexemes<'a> {
	fn default() -> Lexemes<'static> {
		let mut l = Lexemes {
			WS: " \t\r\n",
			PUNCT: "`~!@#$%^&*()+-=[]\\{}|;:,./<>?",
			STR_GRP: "\"'",
			ESCAPE: '\\',
			ESCAPES: HashMap::new(),
			ESC_HEX: 'x',
			ESC_OCT: '0',
			OCT_DIGIT: "01234567",
			DIGIT: "0123456789",
			HEX_DIGIT: "0123456789abcdefABCDEF",
			IDENT_START: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_",
			IDENT: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789",
		};
		l.ESCAPES.insert('n', '\n');
		l.ESCAPES.insert('t', '\t');
		l.ESCAPES.insert('r', '\r');
		l.ESCAPES.insert('"', '"');
		l.ESCAPES.insert('\'', '\'');
		l
	}
}

pub struct Tokenizer<'a, T: Iterator<Item=char>> {
	reader: T,
	pushback: Option<char>,
	lexemes: Lexemes<'a>,
}

#[derive(Debug)]
pub enum Token {
	EOF,
	STRING(String),
	OPER(char),
	NUM(String),
	IDENT(String),
}

fn char_in(s: &str, c: char) -> bool {
	s.chars().find(|&x| x == c).map_or(false, |_| true)
}

impl<'a, T: Iterator<Item=char>> Tokenizer<'a, T> {
	pub fn new(reader: T) -> Tokenizer<'a, T> {
		Tokenizer {
			reader: reader,
			pushback: None,
			lexemes: Lexemes::default(),
		}
	}

	fn push_back(&mut self, c: char) -> bool {
		match (*self).pushback {
			None => {
				(*self).pushback = Some(c);
				true
			},
			Some(_) => {
				false
			}
		}
	}

	fn nextchar(&mut self) -> Option<char> {
		match (*self).pushback {
			Some(c) => {
				(*self).pushback = None;
				Some(c)
			},
			None => self.reader.next(),
		}
	}

	pub fn nexttoken(&mut self) -> Token {
		let mut c = self.nextchar();
		if c == None {
			return Token::EOF;
		}
		while char_in(self.lexemes.WS, c.unwrap()) {
			c = self.nextchar();
			if c == None {
				return Token::EOF;
			}
		}
		let cc = c.unwrap();
		if char_in(self.lexemes.STR_GRP, cc) {
			let termin = cc;
			let mut value = String::new();
			loop {
				let i = self.nextchar();
				if i == None {
					panic!("Unexpected EOF in string");
				}
				let ic = i.unwrap();
				if ic == termin {
					return Token::STRING(value);
				}
				if ic == self.lexemes.ESCAPE {
					let ty = self.nextchar().expect("Unexpected EOF in string escape");
					if ty == self.lexemes.ESC_HEX {
						let mut hval = String::new();
						loop {
							let hc = self.nextchar().expect("Unexpected EOF in hex const");
							if char_in(self.lexemes.HEX_DIGIT, hc) {
								hval.push(hc);
							} else {
								self.push_back(hc);
								break;
							}
						}
						value.push(u8::from_str_radix(&hval, 16).expect("Bad hex escape constant") as char);
						continue
					}
					if ty == self.lexemes.ESC_OCT {
						let mut oval = String::new();
						loop {
							let oc = self.nextchar().expect("Unexpected EOF in oct const");
							if char_in(self.lexemes.OCT_DIGIT, oc) {
								oval.push(oc);
							} else {
								self.push_back(oc);
								break;
							}
						}
						value.push(u8::from_str_radix(&oval, 8).expect("Bad oct escape constant") as char);
						continue
					}
					value.push(*self.lexemes.ESCAPES.get(&ty).unwrap_or(&ty));
					continue
				}
				value.push(ic);
			}
		}
		if char_in(self.lexemes.PUNCT, cc) {
			if cc == '/' {
				let i = self.nextchar();
				if i == None {
					return Token::OPER(cc);
				}
				let ic = i.unwrap();
				if ic == '*' {
					loop {
						let j = self.nextchar();
						match j {
							None => return Token::EOF,
							Some('*') => {
								let k = self.nextchar();
								match k {
									None => return Token::EOF,
									Some('/') => return self.nexttoken(),
									Some(_) => continue,
								}
							},
							Some(_) => continue,
						}
					}
				}
			}
			return Token::OPER(cc);
		}
		if char_in(self.lexemes.DIGIT, cc) {
			let mut num = String::new();
			num.push(cc);
			loop {
				let i = self.nextchar();
				if i == None {
					return Token::NUM(num);
				}
				let ic = i.unwrap();
				if !char_in(self.lexemes.DIGIT, ic) {
					self.push_back(ic);
					return Token::NUM(num);
				}
				num.push(ic);
			}
		}
		if char_in(self.lexemes.IDENT_START, cc) {
			let mut ident = String::new();
			ident.push(cc);
			loop {
				let i = self.nextchar();
				if i == None {
					return Token::IDENT(ident);
				}
				let ic = i.unwrap();
				if !char_in(self.lexemes.IDENT, ic) {
					self.push_back(ic);
					return Token::IDENT(ident);
				}
				ident.push(ic);
			}
		}
		panic!("Not sure what to do with {:?}", c);
	}
}

impl<'a, T: Iterator<Item=char>> Iterator for Tokenizer<'a, T> {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		match self.nexttoken() {
			Token::EOF => None,
			x => Some(x),
		}
	}
}
