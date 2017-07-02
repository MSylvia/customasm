use diagn::Span;
use std::rc::Rc;


#[derive(Debug, Clone)]
pub struct Token
{
	pub span: Span,
	pub kind: TokenKind,
	pub excerpt: Option<String>
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenKind
{
	End,
	Error,
	Whitespace,
	Comment,
	LineBreak,
	Identifier,
	Number,
	String,
	ParenOpen,
	ParenClose,
	BracketOpen,
	BracketClose,
	BraceOpen,
	BraceClose,
	Dot,
	Comma,
	Colon,
	Arrow,
	Hash,
	Equal,
	Plus,
	Minus,
	Asterisk,
	Slash,
	Percent,
	Exclamation,
	Ampersand,
	VerticalBar,
	Circumflex,
	Tilde,
	AmpersandAmpersand,
	VerticalBarVerticalBar,
	EqualEqual,
	ExclamationEqual,
	LessThan,
	LessThanEqual,
	GreaterThan,
	GreaterThanEqual
}


impl TokenKind
{
	fn needs_excerpt(self) -> bool
	{
		self == TokenKind::Identifier ||
		self == TokenKind::Number ||
		self == TokenKind::String
	}
}


pub fn tokenize<S>(src_filename: S, src: &[char]) -> Vec<Token>
where S: Into<String>
{
	let filename = Rc::new(src_filename.into());
	let mut tokens = Vec::new();
	let mut index = 0;
	
	while index < src.len()
	{
		// Decide what are the next token's kind and length.
		let (kind, length) =
			check_for_whitespace(&src[index..]).unwrap_or_else(||
			check_for_comment   (&src[index..]).unwrap_or_else(||
			check_for_identifier(&src[index..]).unwrap_or_else(||
			check_for_number    (&src[index..]).unwrap_or_else(||
			check_for_string    (&src[index..]).unwrap_or_else(||
			check_for_fixed     (&src[index..]).unwrap_or_else(||
			(TokenKind::Error, 1)))))));
		
		// Get the source excerpt for variable tokens (e.g. identifiers).
		let excerpt = match kind.needs_excerpt()
		{
			true => Some(src[index..].iter().cloned().take(length).collect()),
			false => None
		};
		
		let span = Span::new(filename.clone(), index, index + length);
		
		let token = Token
		{
			span: span,
			kind: kind,
			excerpt: excerpt
		};
		
		tokens.push(token);
		
		index += length;
	}
	
	// Add an end token.
	let end_token = Token
	{
		span: Span::new(filename.clone(), index, index),
		kind: TokenKind::End,
		excerpt: None
	};
	
	tokens.push(end_token);
	
	tokens
}


fn check_for_whitespace(src: &[char]) -> Option<(TokenKind, usize)>
{
	let mut length = 0;
	
	if !is_whitespace(src[length])
		{ return None; }
	
	while length < src.len() && is_whitespace(src[length])
		{ length += 1; }
		
	Some((TokenKind::Whitespace, length))
}


fn check_for_comment(src: &[char]) -> Option<(TokenKind, usize)>
{
	let mut length = 0;
	
	if src[length] != ';'
		{ return None; }
	
	while length < src.len() && src[length] != '\n'
		{ length += 1; }
		
	Some((TokenKind::Comment, length))
}


fn check_for_identifier(src: &[char]) -> Option<(TokenKind, usize)>
{
	let mut length = 0;
	
	if !is_identifier_start(src[length])
		{ return None; }
	
	while length < src.len() && is_identifier_mid(src[length])
		{ length += 1; }
		
	Some((TokenKind::Identifier, length))
}


fn check_for_number(src: &[char]) -> Option<(TokenKind, usize)>
{
	let mut length = 0;
	
	if !is_number_start(src[length])
		{ return None; }
	
	while length < src.len() && is_number_mid(src[length])
		{ length += 1; }
		
	Some((TokenKind::Number, length))
}


fn check_for_string(src: &[char]) -> Option<(TokenKind, usize)>
{
	let mut length = 0;
	
	if src[length] != '\"' // "
		{ return None; }
		
	length += 1;
	
	while length < src.len() && src[length] != '\"' // "
		{ length += 1; }
		
	if length >= src.len()
		{ return None; }
		
	if src[length] != '\"' // "
		{ return None; }
		
	Some((TokenKind::String, length))
}


fn check_for_fixed(src: &[char]) -> Option<(TokenKind, usize)>
{
	static OPERATORS: [(&str, TokenKind); 31] =
	[
		("\n", TokenKind::LineBreak),
		("(",  TokenKind::ParenOpen),
		(")",  TokenKind::ParenClose),
		("[",  TokenKind::BracketOpen),
		("]",  TokenKind::BracketClose),
		("{",  TokenKind::BraceOpen),
		("}",  TokenKind::BraceClose),
		(".",  TokenKind::Dot),
		(",",  TokenKind::Comma),
		(":",  TokenKind::Colon),
		("->", TokenKind::Arrow),
		("#",  TokenKind::Hash),
		("+",  TokenKind::Plus),
		("-",  TokenKind::Minus),
		("*",  TokenKind::Asterisk),
		("/",  TokenKind::Slash),
		("%",  TokenKind::Percent),
		("^",  TokenKind::Circumflex),
		("~",  TokenKind::Tilde),
		("&&", TokenKind::AmpersandAmpersand),
		("&",  TokenKind::Ampersand),
		("||", TokenKind::VerticalBarVerticalBar),
		("|",  TokenKind::VerticalBar),
		("==", TokenKind::EqualEqual),
		("=",  TokenKind::Equal),
		("!=", TokenKind::ExclamationEqual),
		("!",  TokenKind::Exclamation),
		("<=", TokenKind::LessThanEqual),
		("<",  TokenKind::LessThan),
		(">=", TokenKind::GreaterThanEqual),
		(">",  TokenKind::GreaterThan)
	];
	
	let maybe_match = OPERATORS.iter().find(|op|
	{
		for (i, c) in op.0.chars().enumerate()
		{
			if i >= src.len() || src[i] != c
				{ return false; }
		}
		true
	});
	
	maybe_match.map(|s| { (s.1, s.0.chars().count()) })
}


fn is_whitespace(c: char) -> bool
{
	c == ' '  ||
	c == '\t' ||
	c == '\r'
}


fn is_identifier_start(c: char) -> bool
{
	(c >= 'a' && c <= 'z') ||
	(c >= 'A' && c <= 'Z') ||
	c == '_'
}


fn is_identifier_mid(c: char) -> bool
{
	(c >= 'a' && c <= 'z') ||
	(c >= 'A' && c <= 'Z') ||
	(c >= '0' && c <= '9') ||
	c == '_'
}


fn is_number_start(c: char) -> bool
{
	(c >= '0' && c <= '9')
}


fn is_number_mid(c: char) -> bool
{
	(c >= 'a' && c <= 'z') ||
	(c >= 'A' && c <= 'Z') ||
	(c >= '0' && c <= '9') ||
	c == '_' ||
	c == '.' ||
	c == '\''
}