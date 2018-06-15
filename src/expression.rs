use super::{Pattern, Identifier, Literal, Function};

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Expression {
    pub data: ExpressionData
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ExpressionData {
    This,
    Array(Array),
    Object(Object),
    Function(Function),
    Unary(Unary),
    Update(Update),
    Binary(Binary),
    Assignment(Assignment),
    Logical(Logical),
    Member(Member),
    Conditional(Conditional),
    Call(Call),
    New(New),
    Sequence(Sequence),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Array {
    pub elements: Vec<Option<Expression>>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Object {
    pub properties: Vec<Property>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Property {
    pub key: PropertyKey,
    pub value: Box<Expression>,
    pub kind: PropertyKind,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum PropertyKey {
    Literal(Literal),
    Identifier(Identifier),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Unary {
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Box<Expression>
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum UnaryOperator {
    /// -
    /// ```js
    /// let x = -1;
    /// ```
    Minus,
    /// +
    /// ```js
    /// let x = +1;
    /// ```
    Plus,
    /// !
    /// ```js
    /// if (!x) return;
    /// ```
    Not,
    /// ~
    /// ```js
    /// let x = ~8;
    /// ```
    Tilde,
    /// typeof
    /// ```js
    /// if (typeof x == 'string') return;
    /// ```
    TypeOf,
    /// void
    /// ```js
    /// if (x == void 0) return;
    /// ```
    Void,
    /// delete
    /// ```js
    /// let x = {x: 1};
    /// delete x[x]
    /// ```
    Delete,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Update {
    pub operator: UpdateOperator,
    pub argument: Box<Expression>,
    pub prefix: bool,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum UpdateOperator {
    /// ++
    Increment,
    /// --
    Decrement,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Binary {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum BinaryOperator {
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// ===
    StrictEqual,
    /// !==
    NotStrictEqual,
    /// <
    LessThan,
    /// <=
    LessThanEqual,
    /// >
    GreaterThan,
    /// >=
    GreaterThanEqual,
    /// <<
    LeftShift,
    /// >>
    RightShift,
    /// >>>
    UnsignedRightShift,
    /// +
    Add,
    /// -
    Subtract,
    /// *
    Multiply,
    /// /
    Divide,
    /// %
    Modulo,
    /// |
    Or,
    /// ^
    XOr,
    /// &
    And,
    /// in
    In,
    /// instanceof
    InstanceOf,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Assignment {
    pub operator: AssignmentOperator,
    pub left: Box<AssignmentData>,
    pub right: Box<Expression>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum AssignmentOperator {
    /// =
    Equal,
    /// +=
    PlusEqual,
    /// -=
    MinusEqual,
    /// *=
    TimesEqual,
    /// /=
    DivideEqual,
    /// %=
    ModEqual,
    /// <<=
    LeftShiftEqual,
    /// >>=
    RightShiftEqual,
    /// >>>=
    UnsignedRightShiftEqual,
    /// |=
    OrEqual,
    /// ^=
    XOrEqual,
    /// &=
    AndEqual,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum AssignmentData {
    Pattern(Pattern),
    Expression(Expression)
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Logical {
    pub operator: LogicalOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum LogicalOperator {
    /// ||
    Or,
    /// &&
    And
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Member {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Conditional {
    pub test: Box<Expression>,
    pub alternate: Box<Expression>,
    pub consequent: Box<Expression>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Call {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct New {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Sequence {
    pub expressions: Vec<Expression>,
}