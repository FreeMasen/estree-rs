use expression::Expression;
use declaration::Declaration;
use pattern::Pattern;
use super::{Identifier, Literal};
pub struct Statement {
    pub data: StatementData,
}

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

pub struct Directive {
    pub expression: Literal,
    pub directive: String,
} 

pub enum Block {
    Plain(Vec<Statement>),
    Function(Vec<Statement>, Vec<Statement>)
}

pub struct With {
    pub object: Expression,
    pub body: Box<Statement>,
}

pub struct Return {
    pub argument: Option<Expression>,
}

pub struct Labeled {
    pub label: Identifier,
    pub body: Box<Statement>,
}

pub struct Break {
    pub label: Option<Identifier>,
}

pub struct Continue {
    pub label: Option<Identifier>,
}

pub struct If {
    pub test: Expression,
    pub consequence: Box<Statement>,
    pub alternate: Box<Option<Statement>>,
}

pub struct Switch {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}

pub struct Throw {
    pub argument: Expression,
}

pub struct Try {
    pub block: Block,
    pub handler: Option<Catch>,
    pub finalizer: Option<Block>,
}

pub struct Catch {
    pub param: Pattern,
    pub body: Block,
}

pub struct While {
    pub test: Expression,
    pub body: Box<Statement>,
}

pub struct DoWhile {
    pub body: Box<Statement>,
    pub test: Expression,
}

pub struct For {
    pub init: ForInit,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
}

pub enum ForInit {
    Var(Declaration),
    Expression(Expression),
    None
}

pub struct ForIn {
    pub left: ForInLeft,
    pub right: Expression,
    pub body: Box<Statement>
}

pub enum ForInLeft {
    Var(Declaration),
    Pattern(Pattern),
}