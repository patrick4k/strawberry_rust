use fxhash::FxHashMap;
use crate::util::util::{ZeroOrMore};


#[derive(Clone)]
pub struct Script {
    pub(crate) declarations: ZeroOrMore<Declaration>,
    pub(crate) body: ZeroOrMore<Expression>
}

#[derive(Clone)]
pub enum FnPublicity { Public, Private, Protected }

#[derive(Clone)]
pub enum Declaration {
    FnDeclaration{ publicity: FnPublicity, id: Identifier, return_type: String, params: ZeroOrMore<Expression>, body: ZeroOrMore<Expression> },
    StateDeclaration{ publicity: FnPublicity, type_name: String, states: ZeroOrMore<State> },
    VariableDeclaration{ id: Identifier, init: Expression },
}


#[derive(Clone)]
pub struct State { id: Identifier, members: ZeroOrMore<Member> }


#[derive(Clone)]
pub struct Member { id: Identifier, type_name: String }


#[derive(Clone)]
pub enum Expression {
    Literal{ value: String },
    Identifier{ id: Identifier },
    BinaryExpression{ left: Box<Self>, operator: String, right: Box<Self> },
    UnaryExpression{ operator: String, right: Box<Self> },
    CallExpression{ callee: Box<Self>, arguments: Vec<Self> },
    MemberExpression{ object: Box<Self>, property: Box<Self> },
    ArrayExpression{ elements: Vec<Self> },
    FunctionExpression{ params: ZeroOrMore<Self>, body: ZeroOrMore<Self> },
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
