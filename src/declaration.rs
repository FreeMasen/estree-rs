use expression::Expression;
use pattern::Pattern;
use super::{Function, Identifier, Class};
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ModuleDeclaration {
    pub kind: DeclData,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DeclData {
    Import(Import),
    Export(Export),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Import {
    pub specifiers: Vec<ImportSpecifier>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ImportSpecifier {
    pub local: Identifier,
    pub imported: Option<Identifier>,
    pub kind: ImportSpecifierKind,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ImportSpecifierKind {
    Normal,
    Default,
    Namespace
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Export {
    pub kind: ExportKind,
    pub declaration: Option<DeclOrExpr>,
    pub specifiers: Option<Vec<Identifier>>,
    pub source: Option<super::Literal>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DeclOrExpr {
    Decl(Declaration),
    Expr(super::Expression),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ExportKind {
    Named(),
    Default,
    All,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ExportSpecifier {
    pub local: Identifier,
    pub exported: Option<Identifier>,
    pub kind: ExportSpecifierKind
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ExportSpecifierKind {
    Normal,
    Default,
    All,
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Declaration {
    pub data: DeclarationData,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DeclarationData {
    Func(FunctionDeclaration),
    Var(VariableDeclaration),
    Class(Class),
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