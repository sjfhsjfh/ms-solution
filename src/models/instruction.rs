use log::error;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    #[default]
    Empty = 0,

    /// Shortcut: A
    SlideL = 1,

    /// Shortcut: D
    SlideR = 2,

    /// Shortcut: W
    Push = 3,

    /// Shortcut: S
    Pull = 4,

    /// Shortcut: Q
    RotateCCW = 5,

    /// Shortcut: E
    RotateCW = 6,

    /// Shortcut: R
    Add = 7,

    /// Shortcut: F
    RemoveH = 8,

    /// Shortcut: X
    Trash = 9,

    /// Shortcut: C
    Output = 10,

    /// Shortcut: V
    ShuntH = 11,
}

impl TryFrom<u8> for Instruction {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::Empty),
            1 => Ok(Instruction::SlideL),
            2 => Ok(Instruction::SlideR),
            3 => Ok(Instruction::Push),
            4 => Ok(Instruction::Pull),
            5 => Ok(Instruction::RotateCCW),
            6 => Ok(Instruction::RotateCW),
            7 => Ok(Instruction::Add),
            8 => Ok(Instruction::RemoveH),
            9 => Ok(Instruction::Trash),
            10 => Ok(Instruction::Output),
            11 => Ok(Instruction::ShuntH),
            v => {
                error!("Invalid instruction value: {}", v);
                Err("Invalid instruction value")
            }
        }
    }
}

impl From<Instruction> for u8 {
    fn from(value: Instruction) -> Self {
        match value {
            Instruction::Empty => 0,
            Instruction::SlideL => 1,
            Instruction::SlideR => 2,
            Instruction::Push => 3,
            Instruction::Pull => 4,
            Instruction::RotateCCW => 5,
            Instruction::RotateCW => 6,
            Instruction::Add => 7,
            Instruction::RemoveH => 8,
            Instruction::Trash => 9,
            Instruction::Output => 10,
            Instruction::ShuntH => 11,
        }
    }
}
