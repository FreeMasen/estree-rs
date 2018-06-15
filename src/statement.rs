use expression::Expression;
use declaration::Declaration;
use pattern::Pattern;
use super::{Identifier, Literal};
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Statement {
    pub data: StatementData,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum StatementData {
    Expression(Expression),
    Directive(Directive),
    Block(Block),
    Empty,
    Debugger,
    With(With),
    Return(Return),
    Labeled(Labeled),
    Break(Break),
    Continue(Continue),
    If(If),
    Switch(Switch),
    Case(SwitchCase),
    Throw(Expression),
    Try(Try),
    Catch(Catch),
    While(While),
    DoWhile(DoWhile),
    For(For),
    ForIn(ForIn),
    Declaration(Declaration)
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Directive {
    pub expression: Literal,
    pub directive: String,
} 
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Block {
    Plain(Vec<Statement>),
    Function(Vec<Statement>, Vec<Statement>)
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct With {
    pub object: Expression,
    pub body: Box<Statement>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Return {
    pub argument: Option<Expression>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Labeled {
    pub label: Identifier,
    pub body: Box<Statement>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Break {
    pub label: Option<Identifier>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Continue {
    pub label: Option<Identifier>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct If {
    pub test: Expression,
    pub consequence: Box<Statement>,
    pub alternate: Box<Option<Statement>>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Switch {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Throw {
    pub argument: Expression,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Try {
    pub block: Block,
    pub handler: Option<Catch>,
    pub finalizer: Option<Block>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Catch {
    pub param: Pattern,
    pub body: Block,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct While {
    pub test: Expression,
    pub body: Box<Statement>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct DoWhile {
    pub body: Box<Statement>,
    pub test: Expression,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct For {
    pub init: ForInit,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ForInit {
    Var(Declaration),
    Expression(Expression),
    None
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ForIn {
    pub left: ForInLeft,
    pub right: Expression,
    pub body: Box<Statement>
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ForInLeft {
    Var(Declaration),
    Pattern(Pattern),
}