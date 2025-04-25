use std::ops::Neg;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// Hexagonal coordinates
///            /
///          +b
///         /  60deg
/// - -a - 0 - +a ->
/// 
/// Origin is at the center of the hexagon
pub struct Hex<T = i32> {
    pub a: T,
    pub b: T,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// CCW positive
pub enum Rotation {
    #[default]
    /// 12 o'clock
    R0,

    /// 10 o'clock
    R60,

    /// 8 o'clock
    R120,

    /// 6 o'clock
    R180,

    /// 4 o'clock
    R240,

    /// 2 o'clock
    R300,
}

impl Neg for Rotation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Rotation::R0 => Rotation::R0,
            Rotation::R60 => Rotation::R300,
            Rotation::R120 => Rotation::R240,
            Rotation::R180 => Rotation::R180,
            Rotation::R240 => Rotation::R120,
            Rotation::R300 => Rotation::R60,
        }
    }
}

impl From<i32> for Rotation {
    fn from(value: i32) -> Self {
        match value.rem_euclid(6) {
            0 => Rotation::R0,
            1 => Rotation::R60,
            2 => Rotation::R120,
            3 => Rotation::R180,
            4 => Rotation::R240,
            5 => Rotation::R300,
            _ => unreachable!(),
        }
    }
}

impl From<Rotation> for i32 {
    fn from(value: Rotation) -> Self {
        match value {
            Rotation::R0 => 0,
            Rotation::R60 => 1,
            Rotation::R120 => 2,
            Rotation::R180 => 3,
            Rotation::R240 => 4,
            Rotation::R300 => 5,
        }
    }
}
