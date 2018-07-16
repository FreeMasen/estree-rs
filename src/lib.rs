pub mod expression;
pub mod statement;
pub mod declaration;
pub mod pattern;

use declaration::{Declaration, ModuleDeclaration};
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
    Pattern(Pattern),
    Super,
    SpreadElement(SpreadElement),
    TemplateElement(TemplateElement),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Identifier(String);

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
    Template(Template)
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegEx {
    pub pattern: String,
    pub flags: String,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Program {
    pub body: Vec<ProgramBodyPart>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ProgramBodyPart {
    Directive(Directive),
    Statement(Statement),
    Declaration(ModuleDeclaration),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Function {
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: FunctionBody,
    pub generator: bool,
    pub async: bool,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct FunctionBody {
    pub body: Vec<FunctionBodyPart>
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum FunctionBodyPart {
    Directive(Directive),
    Statement(Statement),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct SpreadElement {
    argument: Box<Expression>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Template {
    pub quasis: Vec<TemplateElement>,
    pub expression: Vec<Expression>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct TaggedTemplateExpression {
    pub tag: Expression,
    pub quasi: Template,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct TemplateElement {
    pub tail: bool,
    pub value: TemplateElementValue,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct TemplateElementValue {
    pub cooked: Option<String>,
    pub raw: String,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Class {
    pub id: Option<Identifier>,
    pub super_class: Option<Expression>,
    pub body: ClassBody,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ClassBody {
    pub body: Vec<MethodDefinition>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct MethodDefinition {
    pub key: Expression,
    pub value: Expression,
    pub kind: MethodKind,
    pub computed: bool,
    pub is_static: bool,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum MethodKind {
    Constructor,
    Method,
    Get,
    Set,
}