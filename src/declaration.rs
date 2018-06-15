use expression::Expression;
use pattern::Pattern;
use super::{Function, Identifier};
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Declaration {
    pub data: DeclarationData,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DeclarationData {
    Func(FunctionDeclaration),
    Var(VariableDeclaration),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct FunctionDeclaration {
    pub id: Identifier,
    pub func: Function,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct VariableDeclaration {
    pub declaration: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<Expression>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum VariableKind {
    Var,
    Let,
    Const,
}