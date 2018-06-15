use expression::Expression;
use pattern::Pattern;
use super::{Function, Identifier};

pub struct Declaration {
    pub data: DeclarationData,
}

pub enum DeclarationData {
    Func(FunctionDeclaration),
    Var(VariableDeclaration),
}

pub struct FunctionDeclaration {
    pub id: Identifier,
    pub func: Function,
}

pub struct VariableDeclaration {
    pub declaration: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}

pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<Expression>,
}

pub enum VariableKind {
    Var,
    Let,
    Const,
}