
pub struct AST<Rule> {
    pub root: ASTNode<Rule>,
}

pub struct ASTNode<Rule> {
    pub children: Vec<ASTNode<Rule>>,
    pub rule: Rule,
}
