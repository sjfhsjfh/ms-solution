use super::{BOARD_SIZE, Hex, Instruction, Precursor, Rotation};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartType {
    Invalid,

    Input,

    #[deprecated]
    Output,

    Beam,
}

impl TryFrom<i32> for PartType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PartType::Invalid),
            1 => Ok(PartType::Input),
            #[allow(deprecated)]
            2 => Ok(PartType::Output),
            3 => Ok(PartType::Beam),
            _ => Err("Invalid part type"),
        }
    }
}

impl From<PartType> for i32 {
    fn from(value: PartType) -> Self {
        match value {
            PartType::Invalid => 0,
            PartType::Input => 1,
            #[allow(deprecated)]
            PartType::Output => 2,
            PartType::Beam => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Part {
    pub part_type: PartType,
    pub pos: Hex,
    pub rot: Rotation,
    pub precursor: Option<Precursor>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum PartEnum {
    Input(Input),

    #[deprecated]
    Output,

    Beam(Beam),
}

impl PartEnum {
    pub fn part_type(&self) -> PartType {
        match self {
            PartEnum::Input(_) => PartType::Input,
            #[allow(deprecated)]
            PartEnum::Output => PartType::Output,
            PartEnum::Beam(_) => PartType::Beam,
        }
    }
}

impl From<Input> for PartEnum {
    fn from(input: Input) -> Self {
        PartEnum::Input(input)
    }
}
impl From<Beam> for PartEnum {
    fn from(beam: Beam) -> Self {
        PartEnum::Beam(beam)
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub precursor: Precursor,
    pub pos: Hex,
    pub rotation: Rotation,
}

#[derive(Debug, Clone)]
pub struct Beam {
    pub beam_id: u8,
    pub pos: Hex,
}

impl Beam {
    /// For consistency check
    pub fn rotation(&self) -> Rotation {
        Rotation::from(-(self.beam_id as i32))
    }

    /// Must on the border of the puzzle plane
    pub fn is_valid_pos(&self) -> bool {
        let a = self.pos.a;
        let b = self.pos.b;
        let board_size = BOARD_SIZE as i32 + 1;
        if a < -board_size {
            false
        } else if a == -board_size {
            b >= 0 && b <= board_size
        } else if a <= 0 {
            b == board_size || (a + b) == -board_size
        } else if a < board_size {
            b == -board_size || (a + b) == board_size
        } else if a == board_size {
            b >= -board_size && b <= 0
        } else {
            false
        }
    }
}
