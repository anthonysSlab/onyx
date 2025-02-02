use std::fmt::Formatter;
use std::num::NonZeroU8;

use colored::Colorize;

use crate::span::Span;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum TokenKind {
	Identifier,

	KWExport,
	KWExtern,
	KWImpl,
	KWStatic,
	KWLet,
	KWRet,
	KWType,

	FloatLiteral,

	BinaryIntLiteral,
	OctalIntLiteral,
	DecimalIntLiteral,
	HexadecimalIntLiteral,

	StringLiteral,
	CharLiteral,

	Tilde,
	Bang,
	At,
	Pound,
	Dollar,
	Percent,
	Caret,
	CaretCaret,
	Ampersand,
	AmpersandAmpersand,
	Star,
	LParen,
	RParen,
	Minus,
	Underscore,
	Equals,
	Plus,
	LBracket,
	RBracket,
	LBrace,
	RBrace,
	Pipe,
	PipePipe,
	Semicolon,
	Colon,
	Comma,
	Dot,
	Slash,
	Question,
	ArrowLeft,
	ArrowRight,
	FatArrowRight,
	GreaterThan,
	GreaterThanEquals,
	EqualsEquals,
	LessThan,
	LessThanEquals,
	NotEquals,
	ShiftLeft,
	ShiftRight,
	Apostrophe,

	EOF,
}

#[derive(Debug, Copy, Clone)]
pub struct Token<'source> {
	pub kind: TokenKind,
	pub span: Span,
	pub text: &'source str, // TODO: remove text, just slice from source with span
}

impl std::fmt::Display for Token<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Token({:?}, {}", self.kind, 
			format!("{}-{}", self.span.start, self.span.end).bright_black())?;
		if !self.text.is_empty() { write!(f, ", {}", format!("{:?}", self.text).green())?; }
		write!(f, ")")
	}
}

impl TokenKind {
	pub fn pbind_power(self) -> Option<NonZeroU8> {
		unsafe {
			Some(NonZeroU8::new_unchecked(match self {
				Self::Minus => 5,
				_ => return None,
			}))
		}
	}

	pub fn ibind_power(self) -> Option<NonZeroU8> {
		unsafe {
			Some(NonZeroU8::new_unchecked(match self {
				Self::Star | Self::Slash | Self::Percent => 2,
				Self::Plus | Self::Minus => 1,
				_ => return None,
			}))
		}
	}

	pub fn sbind_power(self) -> Option<NonZeroU8> {
		unsafe {
			Some(NonZeroU8::new_unchecked(match self {
				Self::LParen | Self::LBracket => 14,
				_ => return None,
			}))
		}
	}

	pub fn is_delim(self) -> bool {
		matches!(self, Self::Semicolon | Self::RParen | Self::RBracket | Self::RBrace | Self::Comma)
	}
}
