use fxhash::FxHashMap;
use crate::util::util::{OneOrMore, ZeroOrMore};

#[derive(Clone)]
pub enum Script {
    Script { includes: ZeroOrMore<Include>,body: ZeroOrMore<Expression> }
}

#[derive(Clone)]
pub enum Include {
    Header { path: OneOrMore<String> }
}

#[derive(Clone)]
pub enum Expression {
    Literal{ value: String },
    Identifier{ id: Identifier },
    BinaryExpression{ left: Box<Self>, operator: String, right: Box<Self> },
    UnaryExpression{ operator: String, right: Box<Self> },
    CallExpression{ callee: Box<Self>, arguments: Vec<Expression> },
    MemberExpression{ object: Box<Self>, property: Box<Self> },
    ArrayExpression{ elements: Vec<Expression> },
    FunctionExpression{ params: ZeroOrMore<Expression>, body: ZeroOrMore<Expression> },
}

#[derive(Clone)]
pub enum Identifier {
    Identifier{ name: String },
    DotIdentifier{ object: Box<Self>, property: Box<Self> },
}

#[derive(Clone)]
pub enum Literals {
    StringLiteral{ value: String },
    DoubleLiteral{ value: f32 },
    IntLiteral{ value: i32 },
    BooleanLiteral{ value: bool },
    ArrayLiteral{ value: ZeroOrMore<Expression> },
    HashLiteral{ value: FxHashMap<String, Expression> },
}
