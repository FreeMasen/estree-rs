use expression::Expression;
use pattern::Pattern;
use super::{Function, Identifier};

pub struct Declaration {
    data: DeclarationData,
}

pub enum DeclarationData {
    Func(FunctionDeclaration),
    Var(VariableDeclaration),
}

pub struct FunctionDeclaration {
    id: Identifier,
    func: Function,
}

pub struct VariableDeclaration {
    declaration: Vec<VariableDeclarator>,
    kind: VariableKind,
}

pub struct VariableDeclarator {
    id: Pattern,
    init: Option<Expression>,
}

pub enum VariableKind {
    Var,
    Let,
    Const,
}