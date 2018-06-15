pub mod expression;
pub mod statement;
pub mod declaration;
pub mod pattern;

use declaration::Declaration;
use expression::Expression;
use pattern::Pattern;
use statement::{Statement, Directive};
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Node {
    pub data: NodeData,
}
#[cfg_attr(feature = "debug", derive(Debug))]
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
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Identifier {
    pub name: String,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Literal {
    pub value: LiteralValue,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum LiteralValue {
    String(String),
    Bool(bool),
    Null,
    Number(f32),
    RegEx(RegEx),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegEx {
    pub pattern: String,
    pub flags: String,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Program {
    pub id: Identifier,
    pub params: Vec<Pattern>,
    pub body: ProgramBody,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ProgramBody {
    Directive(Directive),
    Statement(Statement)
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Function {
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: FunctionBody,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct FunctionBody {
    pub body: Vec<FunctionBodyData>
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum FunctionBodyData {
    Directive(Directive),
    Statement(Statement),
}