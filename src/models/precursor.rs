#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precursor {
    Invalid,
    Water,
    Ammonia,
    HydrochloricAcid,
    SulfuricAcid,
    Acetone,
    Cyclohexane,
    Methanol,
    EthyleneGlycol,
    Propene,
    AceticAcid,
    Benzene,
    Carbamide,
    PhosphoricAcid,
    HydrofluoricAcid,
    Hydrazine,
    Butanone,
    ThionylChloride,
    Dioxane,
    Toluene,
    FormicAcid,
    Butylene,
    Isobutane,
    Triazine,
}

impl TryFrom<i32> for Precursor {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Precursor::Invalid),
            1 => Ok(Precursor::Water),
            2 => Ok(Precursor::Ammonia),
            3 => Ok(Precursor::HydrochloricAcid),
            4 => Ok(Precursor::SulfuricAcid),
            5 => Ok(Precursor::Acetone),
            6 => Ok(Precursor::Cyclohexane),
            7 => Ok(Precursor::Methanol),
            8 => Ok(Precursor::EthyleneGlycol),
            9 => Ok(Precursor::Propene),
            10 => Ok(Precursor::AceticAcid),
            11 => Ok(Precursor::Benzene),
            12 => Ok(Precursor::Carbamide),
            13 => Ok(Precursor::PhosphoricAcid),
            14 => Ok(Precursor::HydrofluoricAcid),
            15 => Ok(Precursor::Hydrazine),
            16 => Ok(Precursor::Butanone),
            17 => Ok(Precursor::ThionylChloride),
            18 => Ok(Precursor::Dioxane),
            19 => Ok(Precursor::Toluene),
            20 => Ok(Precursor::FormicAcid),
            21 => Ok(Precursor::Butylene),
            22 => Ok(Precursor::Isobutane),
            23 => Ok(Precursor::Triazine),
            _ => Err("Invalid precursor value"),
        }
    }
}

impl From<Precursor> for i32 {
    fn from(value: Precursor) -> Self {
        match value {
            Precursor::Invalid => 0,
            Precursor::Water => 1,
            Precursor::Ammonia => 2,
            Precursor::HydrochloricAcid => 3,
            Precursor::SulfuricAcid => 4,
            Precursor::Acetone => 5,
            Precursor::Cyclohexane => 6,
            Precursor::Methanol => 7,
            Precursor::EthyleneGlycol => 8,
            Precursor::Propene => 9,
            Precursor::AceticAcid => 10,
            Precursor::Benzene => 11,
            Precursor::Carbamide => 12,
            Precursor::PhosphoricAcid => 13,
            Precursor::HydrofluoricAcid => 14,
            Precursor::Hydrazine => 15,
            Precursor::Butanone => 16,
            Precursor::ThionylChloride => 17,
            Precursor::Dioxane => 18,
            Precursor::Toluene => 19,
            Precursor::FormicAcid => 20,
            Precursor::Butylene => 21,
            Precursor::Isobutane => 22,
            Precursor::Triazine => 23,
        }
    }
}
