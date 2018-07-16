pub trait Node {
    fn kind(&self) -> NodeKind;
    fn location(&self) -> Location;
}
pub enum NodeKind {
    Identifier,
    Literal(LiteralKind),
    Program,
    Function(FunctionKind),
    Statement(StatementKind),
    Decl(DeclKind),
    Expr(ExprKind),
    Pattern(PatternKind),
    Super,
    SpreadElement,
    Template(TemplateKind),
    CatchClause
}
pub enum LiteralKind {
    String,
    Boolean,
    Null,
    Number,
    RegEx,
}
pub enum FunctionKind {
    Expr,
    Decl,
}
pub enum StatementKind {
    Expr,
    Block,
    Empty,
    Debugger,
    With,
    Return,
    Labeled,
    Break,
    Continue,
    If,
    Switch,
    Throw,
    Try,
    While,
    DoWhile,
    For,
    ForIn,
    ForOf,
}
pub enum DeclKind {
    Func,
    Var,
}
pub enum ExprKind {
    This,
    Array,
    Object,
    Function,
    Unary,
    Update,
    Binary,
    Assignment,
    Logical,
    Member,
    Conditional,
    Call,
    New,
    Sequence,
}
pub enum PatternKind {

}
pub enum TemplateKind {

}
#[derive(Clone)]
pub struct Location {
    pub start: Position,
    pub end: Position
}
impl Location {
    pub fn new(start: Position, end: Position) -> Self {
        Location {
            start,
            end
        }
    }
}
#[derive(Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}
impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position {
            line,
            column,
        }
    }
}
pub struct Identifier {
    pub name: String,
    location: Location,
}
impl Node for Identifier {
    fn kind(&self) -> NodeKind {
        NodeKind::Identifier
    }
    fn location(&self) -> Location {
        self.location.clone()
    }
}
pub trait Literal<T: Clone> {
    fn value(&self) -> T;
}
#[derive(Clone)]
struct StringLit {
    quote: QuoteKind,
    content: String,
}
#[derive(Clone)]
pub enum QuoteKind {
    Single,
    Double,
}

pub trait Number<T: Clone>: Literal<T> {
    fn sign(&self) -> Option<Sign>;
}
#[derive(Clone)]
pub enum Sign {
    Positive,
    Negative,
}
impl<'a> Into<char> for &'a Sign {
    fn into(self) -> char {
        match self {
            &Sign::Positive => '+',
            &Sign::Negative => '-',
        }
    }
}
#[derive(Clone)]
pub enum NumberLiteral {
    Binary(Sign, CharCase, usize),
    Octal(Sign, CharCase, usize),
    Hex(Sign, CharCase, usize),
    Decimal(DecimalLiteral)
}
impl Literal<String> for NumberLiteral {
    fn value(&self) -> String {
        let mut ret = String::new();
        match self {
            &NumberLiteral::Binary(sign, case, value) => {
                if let Some(ref s) = sign {
                    ret.push(s.into());
                }
                ret.push(match case {
                    CharCase::Upper => 'B',
                    CharCase::Lower => 'b',
                });
                
            }
            &NumberLiteral::Octal(sign, case, value) => {

            }
            &NumberLiteral::Hex(sign, case, value) => {

            }
            &NumberLiteral::Decimal(sign, case, value) => {

            }
        }
        ret
    }
}
#[derive(Clone)]
pub struct DecimalLiteral {
    sign: Option<Sign>,
    pub integer: Option<usize>,
    pub remainder: Option<usize>,
    pub exp: Option<Exponent>,
}
#[derive(Clone)]
pub struct Exponent {
    pub case: CharCase,
    pub value: usize,
}
#[derive(Clone)]
pub enum CharCase {
    Upper,
    Lower,
}