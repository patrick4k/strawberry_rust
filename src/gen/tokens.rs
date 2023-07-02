
pub enum Token {

	String(String),
	Digit(String),
	Int(String),
	Letter(String),
	Double(String),
	ScientificNotation(String),
	Identifier(String),
	Assignment(String),
	Semicolon(String),
	Comma(String),
	ParenthesisOpen(String),
	ParenthesisClose(String),
	CurlyBracketOpen(String),
	CurlyBracketClose(String),
	SquareBracketOpen(String),
	SquareBracketClose(String),
	Period(String),
	Colon(String),
	Minus(String),
	Plus(String),
	Multiply(String),
	Divide(String),
	Modulo(String),
	Exponent(String),
	Not(String),
	BitwiseAnd(String),
	BitwiseOr(String),
	BitwiseXor(String),
	BitwiseNot(String),
	ShiftLeft(String),
	ShiftRight(String),

}

impl Token {

    pub fn from(name: &str, value: String) -> Token {
        match name {
			"String" => Token::String(value),
			"Digit" => Token::Digit(value),
			"Int" => Token::Int(value),
			"Letter" => Token::Letter(value),
			"Double" => Token::Double(value),
			"ScientificNotation" => Token::ScientificNotation(value),
			"Identifier" => Token::Identifier(value),
			"Assignment" => Token::Assignment(value),
			"Semicolon" => Token::Semicolon(value),
			"Comma" => Token::Comma(value),
			"ParenthesisOpen" => Token::ParenthesisOpen(value),
			"ParenthesisClose" => Token::ParenthesisClose(value),
			"CurlyBracketOpen" => Token::CurlyBracketOpen(value),
			"CurlyBracketClose" => Token::CurlyBracketClose(value),
			"SquareBracketOpen" => Token::SquareBracketOpen(value),
			"SquareBracketClose" => Token::SquareBracketClose(value),
			"Period" => Token::Period(value),
			"Colon" => Token::Colon(value),
			"Minus" => Token::Minus(value),
			"Plus" => Token::Plus(value),
			"Multiply" => Token::Multiply(value),
			"Divide" => Token::Divide(value),
			"Modulo" => Token::Modulo(value),
			"Exponent" => Token::Exponent(value),
			"Not" => Token::Not(value),
			"BitwiseAnd" => Token::BitwiseAnd(value),
			"BitwiseOr" => Token::BitwiseOr(value),
			"BitwiseXor" => Token::BitwiseXor(value),
			"BitwiseNot" => Token::BitwiseNot(value),
			"ShiftLeft" => Token::ShiftLeft(value),
			"ShiftRight" => Token::ShiftRight(value),

            _ => panic!("Invalid token: {}", name)
        }
    }

}
