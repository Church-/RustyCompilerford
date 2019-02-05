use lib::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
	input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
	pub fn new(input_str: &'a str) -> Lexer<'a> {
		Lexer { input: input_str.chars().peekable() }
	}

	fn read_char(&mut self) -> Option<char> {
		self.input.next()
	}
	
	fn peek_char(&mut self) -> Option<&char> {
		self.input.peek()
	}

	fn read_ident(&mut self, c: char) -> String {
		let mut ident = String::new();
		ident.push(c);
		while let Some(&c) = self.peek_char() {
			match c.is_alphabetic() {
				true => ident.push(self.read_char().unwrap()),
				false => break,
			}
		}
		return ident
	}

	fn read_number(&mut self, c: char) -> String {
		let mut number = String::new();
		number.push(c);
		while let Some(&c) = self.peek_char() {
			match c.is_digit(10) {
				true => number.push(self.read_char().unwrap()),
				false => break,
			}
		}
		return number
	}

	fn skip_whitespace(&mut self) {
		while let Some(&c) = self.peek_char() {
			match c.is_whitespace() {
				true => { let _ = self.read_char(); },
				false => break				
			}
		}
	}

	fn check_keyword(ident: String) -> Token {
		match ident.as_str() {
			"fn" => Token::Function,
			"if" => Token::If,
			"else" => Token::Else,
			"true" => Token::True,
			"false" => Token::False,
			"return" => Token::Return,
			"let" => Token::Let,
			"while" => Token::While,
			_ => Token::Ident(ident)
		}
	}

	fn next_token(&mut self) -> Option<Token> {
		self.skip_whitespace();
		if let Some(c) = self.read_char() {
			match c {
				'!' => {
					if let Some(&'=') = self.peek_char() {
							self.read_char();
							Some(Token::NotEq)
						}
						else {
							Some(Token::Bang)
						}
					},
				'=' => {
					if let Some(&'=') = self.peek_char() {
							self.read_char();
							Some(Token::Eq)
						}
						else {
							Some(Token::Assign)
						}
					},
				'(' => Some(Token::LParen),
				')' => Some(Token::RParen),
				'{' => Some(Token::LBrace),
				'}' => Some(Token::RBrace),
				',' => Some(Token::Comma),
				';' => Some(Token::Semicolon),
				'>' => Some(Token::Gt),
				'<' => Some(Token::Lt),
				'+' => Some(Token::Plus),
				'-' => Some(Token::Minus),
				'*' => Some(Token::Asterisk),
				'/' => Some(Token::Slash),
				_ => {
					if c.is_alphabetic() {
						Some(Token::Ident(self.read_ident(c)))
					}
					else if c.is_numeric() {
						Some(Token::Int(self.read_number(c)))
					}
					else {
						Some(Token::Illegal(c))
					}
				}
			}
		}
		else {
			None
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;
	fn next(&mut self) -> Option<Token> {
		self.next_token()
	}
}