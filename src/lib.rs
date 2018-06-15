pub mod expression;
pub mod statement;
pub mod declaration;
pub mod pattern;

use declaration::Declaration;
use expression::Expression;
use pattern::Pattern;
use statement::{Statement, Directive};

pub struct Node {
    pub data: NodeData,
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
    pub name: String,
}

pub struct Literal {
    pub value: LiteralValue,
}

pub enum LiteralValue {
    String(String),
    Bool(bool),
    Null,
    Number(f32),
    RegEx(RegEx),
}

pub struct RegEx {
    pub pattern: String,
    pub flags: String,
}

pub struct Program {
    pub id: Identifier,
    pub params: Vec<Pattern>,
    pub body: ProgramBody,
}

pub enum ProgramBody {
    Directive(Directive),
    Statement(Statement)
}

pub struct Function {
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: FunctionBody,
}

pub struct FunctionBody {
    pub body: Vec<FunctionBodyData>
}

pub enum FunctionBodyData {
    Directive(Directive),
    Statement(Statement),
}