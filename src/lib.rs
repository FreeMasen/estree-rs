pub mod expression;
pub mod statement;
pub mod declaration;
pub mod pattern;

use declaration::Declaration;
use expression::Expression;
use pattern::Pattern;
use statement::{Statement, Directive};

pub struct Node {
    data: NodeData,
}

pub enum NodeData {
    Identifier(Identifier),
    Literal(Literal),
    Program(Program),
    Function(Function),
    Statement(Statement),
    Declaration(Declaration),
    Expression(Expression),
    Pattern(Pattern)
}

pub struct Identifier {
    name: String,
}

pub struct Literal {
    value: LiteralValue,
}

pub enum LiteralValue {
    String(String),
    Bool(bool),
    Null,
    Number(f32),
    RegEx(RegEx),
}

pub struct RegEx {
    pattern: String,
    flags: String,
}

pub struct Program {
    id: Identifier,
    params: Vec<Pattern>,
    body: ProgramBody,
}

pub enum ProgramBody {
    Directive(Directive),
    Statement(Statement)
}

pub struct Function {
    id: Option<Identifier>,
    params: Vec<Pattern>,
    body: FunctionBody,
}

pub struct FunctionBody {
    body: Vec<FunctionBodyData>
}

pub enum FunctionBodyData {
    Directive(Directive),
    Statement(Statement),
}