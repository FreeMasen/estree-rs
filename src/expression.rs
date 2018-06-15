use super::{Pattern, Identifier, Literal, Function};

pub struct Expression {
    data: ExpressionData
}

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

pub struct Array {
    elements: Vec<Option<Expression>>,
}

pub struct Object {
    properties: Vec<Property>,
}

pub struct Property {
    key: PropertyKey,
    value: Box<Expression>,
    kind: PropertyKind,
}

pub enum PropertyKey {
    Literal(Literal),
    Identifier(Identifier),
}

pub enum PropertyKind {
    Init,
    Get,
    Set,
}

pub struct Unary {
    operator: UnaryOperator,
    prefix: bool,
    argument: Box<Expression>
}

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

pub struct Update {
    operator: UpdateOperator,
    argument: Box<Expression>,
    prefix: bool,
}

pub enum UpdateOperator {
    /// ++
    Increment,
    /// --
    Decrement,
}

pub struct Binary {
    operator: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>,
}

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

pub struct Assignment {
    operator: AssignmentOperator,
    left: Box<AssignmentData>,
    right: Box<Expression>,
}

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

pub enum AssignmentData {
    Pattern(Pattern),
    Expression(Expression)
}

pub struct Logical {
    operator: LogicalOperator,
    left: Box<Expression>,
    right: Box<Expression>,
}

pub enum LogicalOperator {
    /// ||
    Or,
    /// &&
    And
}

pub struct Member {
    object: Box<Expression>,
    property: Box<Expression>,
    computed: bool,
}

pub struct Conditional {
    test: Box<Expression>,
    alternate: Box<Expression>,
    consequent: Box<Expression>,
}

pub struct Call {
    callee: Box<Expression>,
    arguments: Vec<Expression>,
}

pub struct New {
    callee: Box<Expression>,
    arguments: Vec<Expression>,
}

pub struct Sequence {
    expressions: Vec<Expression>,
}