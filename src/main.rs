mod lib;
use lib::lexer::Lexer;
use lib::parser::Parser;

fn main() {
	let mut l = Lexer::new("let f = fn(1+1)");
	while let Some(c) = l.next() {
		println!("{:?}",c);
	}
}
