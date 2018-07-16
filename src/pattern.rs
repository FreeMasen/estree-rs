use super::{Identifier};
use expression::{Property, Expression};
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Pattern {
    pub data: PatternData,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum PatternData {
    Identifier(Identifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
    Rest(RestElement),
    Assignment(AssignmentPattern),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ObjectPattern {
    pub properties: Vec<PropOrRest>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum PropOrRest {
    Prop(Property),
    Rest(RestElement),
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ArrayPattern {
    pub elements: Option<Box<Pattern>>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RestElement {
    pub argument: Box<Pattern>,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct AssignmentPattern {
    pub left: Box<Pattern>,
    pub right: Expression,
}