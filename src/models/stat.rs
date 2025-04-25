#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatField {
    Cycles,
    Modules,
    Symbols,
}

impl TryFrom<u32> for StatField {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StatField::Cycles),
            1 => Ok(StatField::Modules),
            2 => Ok(StatField::Symbols),
            _ => Err("Invalid entity type"),
        }
    }
}

impl From<StatField> for i32 {
    fn from(value: StatField) -> Self {
        match value {
            StatField::Cycles => 0,
            StatField::Modules => 1,
            StatField::Symbols => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub cycles: u32,
    pub modules: u32,
    pub symbols: u32,
}
