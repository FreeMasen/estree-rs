use expression::Expression;
use declaration::Declaration;
use pattern::Pattern;
use super::{Identifier, Literal};
pub struct Statement {
    data: StatementData,
}

enum StatementData {
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
    expression: Literal,
    directive: String,
} 

pub enum Block {
    Plain(Vec<Statement>),
    Function(Vec<Statement>, Vec<Statement>)
}

pub struct With {
    object: Expression,
    body: Box<Statement>,
}

pub struct Return {
    argument: Option<Expression>,
}

pub struct Labeled {
    label: Identifier,
    body: Box<Statement>,
}

pub struct Break {
    label: Option<Identifier>,
}

pub struct Continue {
    label: Option<Identifier>,
}

pub struct If {
    test: Expression,
    consequence: Box<Statement>,
    alternate: Box<Option<Statement>>,
}

pub struct Switch {
    discriminant: Expression,
    cases: Vec<SwitchCase>,
}

pub struct SwitchCase {
    test: Option<Expression>,
    consequent: Vec<Statement>,
}

pub struct Throw {
    argument: Expression,
}

pub struct Try {
    block: Block,
    handler: Option<Catch>,
    finalizer: Option<Block>,
}

pub struct Catch {
    param: Pattern,
    body: Block,
}

pub struct While {
    test: Expression,
    body: Box<Statement>,
}

pub struct DoWhile {
    body: Box<Statement>,
    test: Expression,
}

pub struct For {
    init: ForInit,
    test: Option<Expression>,
    update: Option<Expression>,
    body: Box<Statement>,
}

pub enum ForInit {
    Var(Declaration),
    Expression(Expression),
    None
}

pub struct ForIn {
    left: ForInLeft,
    right: Expression,
    body: Box<Statement>
}

pub enum ForInLeft {
    Var(Declaration),
    Pattern(Pattern),
}