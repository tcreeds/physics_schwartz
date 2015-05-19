extern crate itertools;

extern crate std;

use itertools::Itertools;

macro_rules! with {
	($x:expr => $y:pat) => (
		match $x.expect("Unexpected end of stream.") {
			$y => (),
			_ => panic!(concat!("Expected ", stringify!($y), ".")),
		}	
	)
}

#[derive(Clone, Debug)]
pub enum Token {
	Operator(String),
	Assign,
	Ident(String),
	Number(f64),
	OpenParen,
	CloseParen,
	Comma,
	EoL,
}


#[derive(Clone)]
pub struct Tokenizer<'a> {
	pub chars: std::str::Chars<'a>,	
}

impl<'a> Tokenizer<'a> {
	pub fn new(base: & 'a str) -> Self {
		Tokenizer {
			chars: base.chars(),
		}
	}
}

impl<'a> Iterator for Tokenizer<'a> {
	type Item = Token;
	fn next(&mut self) -> Option<Token> {
		match self.chars.clone().peekable().peek() {
			None => Some(Token::EoL),
			Some(c) => {
				match *c {
					'0' ... '9' => { 
						Some(Token::Number(self.chars.take_while_ref(|c| c.is_numeric() || *c == '.' ).collect::<String>().parse().unwrap()))
					},
					'(' => { self.chars.next(); Some(Token::OpenParen) },
					')' => { self.chars.next(); Some(Token::CloseParen) },
					',' => { self.chars.next(); Some(Token::Comma) },
					'=' => { self.chars.next(); Some(Token::Assign) },
					'a' ... 'z' | 'A' ... 'Z' | '_' | '.'	=> Some(Token::Ident(self.chars.take_while_ref(|c| match *c { 'a' ... 'z' | 'A' ... 'Z' | '_' | '.' => true, _ => false, }).collect())),
					'\n' | '\r' => Some(Token::EoL),
					c if c.is_whitespace() => { self.chars.next(); self.next() },
					_ => Some(Token::Operator(self.chars.take_while_ref(|c| !c.is_whitespace()).collect())),
					
				}
			}
		}
	}	
}

#[derive(Clone)]
pub struct Outputter<I> {
	pub stuff: I,
}

impl<I> Iterator for Outputter<I> where I: Iterator, I::Item: std::fmt::Debug {
	type Item = I::Item;
	fn next(&mut self) -> Option<Self::Item> {
		let ret = self.stuff.next();
		println!("{:?}", ret);
		ret
	}	
}


// v = 2 * pow(4, 2 + y)
// expr := value [op exp]
// expr := 

#[derive(Debug)]
pub enum Expr {
	Number(f64),
	Variable(String),
	Call(String, Vec<Expr>),
	Binary(Box<Expr>, String, Box<Expr>),
}
#[derive(Debug)]
pub enum Line {
	Assign(String, Expr),
}
pub fn parse_line<I>(toks: & mut I) -> Line where I: Iterator<Item=Token> + Clone {
	if let Token::Ident(name) = toks.next().unwrap() {
		with!(toks.next() => Token::Assign);
		let expr = parse_expr(toks);
		Line::Assign(name, expr)
	} else {
		panic!("Expected Ident while parsing line.");
	}
}

fn parse_value<I>(toks: & mut I) -> Expr where I: Iterator<Item=Token> + Clone {
	let mut look_ahead = toks.clone().peekable();
	match look_ahead.peek().expect("Unexpected end of stream.").clone() {
		Token::Ident(name) => {
			look_ahead.next();
			match *look_ahead.peek().expect("Unexpected end of stream.") {
				Token::OpenParen => {
					parse_func(toks)
				},
				_ => {
					with!(toks.next() => Token::Ident(_));
					Expr::Variable(name)
				}
			}
		},
		Token::Number(num) => {
			with!(toks.next() => Token::Number(_));
			Expr::Number(num)
		}, 
		Token::OpenParen => {
			with!(toks.next() => Token::OpenParen);
			let collected_within: Vec<_> = toks.by_ref().take_while(|tok| if let Token::CloseParen = *tok { false } else { true }).collect();
			parse_expr(& mut collected_within.iter().cloned())
		},
		x => panic!(format!("Attempted to parse value, unexpected token found: {:?}.", x)),
	}
}
fn precedence(op: &String) -> u32 {
	match &op[..] {
		"" => 0,
		"+" | "-" => 1,
		"*" | "/" => 2,
		_ => 3,
	}
}
fn parse_expr<I>(toks: & mut I) -> Expr where I: Iterator<Item=Token> + Clone {
	let mut ops: Vec<String> = vec![];	
	let mut exprs: Vec<Expr> = vec![];
	'shunt: loop {
		exprs.push(parse_value(toks));
		match toks.clone().peekable().next() {
			None => break 'shunt,
			Some(tok) => {
				match tok {
					Token::Operator(op) => { 
						toks.next(); 
						match ops.last().clone() {
							None => ops.push(op),
							Some(_) => {
								while precedence(&op) < precedence(ops.last().unwrap_or(&"".to_string())) {
									let top_op = ops.pop().unwrap();
									let lhs = exprs.pop().unwrap();
									let rhs = exprs.pop().unwrap();
									exprs.push(Expr::Binary(
										Box::new(lhs),
										top_op,
										Box::new(rhs),
									));
								}
								ops.push(op);
							}
						}
					},
					_ => break 'shunt,
				}
			}
		}
	}

	'construct: loop {
		if let Some(op) = ops.pop()
		{
			let lhs = exprs.pop().unwrap();
			let rhs = exprs.pop().unwrap();
			exprs.push(Expr::Binary(
				Box::new(lhs),
				op,
				Box::new(rhs),
			));
		} else {
			break 'construct;
		}
	}
	exprs.pop().unwrap()
}

fn parse_func<I>(toks: & mut I) -> Expr where I: Iterator<Item=Token> + Clone {
	if let Token::Ident(function_name) = toks.next().expect("Unexpected end of stream in func.") {
		with!(toks.next() => Token::OpenParen);
		let mut items: Vec<Expr> = vec![];
		'expr: loop {
			match *toks.clone().peekable().peek().expect("Unexpected end of stream in args.") {
				Token::CloseParen => {
					break 'expr;
				}, 
				Token::Comma => { toks.next(); continue 'expr },
				_ => items.push(parse_expr(toks)),
			}
		}
		Expr::Call(function_name, items)
	} else {
		panic!("Expected Ident at beginning of Function Call");
	}
} 