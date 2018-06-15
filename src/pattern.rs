use super::Identifier;

pub struct Pattern {
    data: PatternData,
}

pub enum PatternData {
    Identifier(Identifier),
}