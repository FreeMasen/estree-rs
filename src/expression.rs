use super::{Pattern, Identifier, Literal, Function};

pub struct Expression {
    pub data: ExpressionData
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
    pub elements: Vec<Option<Expression>>,
}

pub struct Object {
    pub properties: Vec<Property>,
}

pub struct Property {
    pub key: PropertyKey,
    pub value: Box<Expression>,
    pub kind: PropertyKind,
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
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Box<Expression>
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
    pub operator: UpdateOperator,
    pub argument: Box<Expression>,
    pub prefix: bool,
}

pub enum UpdateOperator {
    /// ++
    Increment,
    /// --
    Decrement,
}

pub struct Binary {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
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
    pub operator: AssignmentOperator,
    pub left: Box<AssignmentData>,
    pub right: Box<Expression>,
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
    pub operator: LogicalOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub enum LogicalOperator {
    /// ||
    Or,
    /// &&
    And
}

pub struct Member {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool,
}

pub struct Conditional {
    pub test: Box<Expression>,
    pub alternate: Box<Expression>,
    pub consequent: Box<Expression>,
}

pub struct Call {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

pub struct New {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

pub struct Sequence {
    pub expressions: Vec<Expression>,
}