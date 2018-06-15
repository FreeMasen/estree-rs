use super::Identifier;

pub struct Pattern {
    pub data: PatternData,
}

pub enum PatternData {
    Identifier(Identifier),
}