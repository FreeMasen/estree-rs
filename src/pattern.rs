use super::Identifier;
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Pattern {
    pub data: PatternData,
}
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum PatternData {
    Identifier(Identifier),
}