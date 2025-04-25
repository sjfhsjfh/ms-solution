use log::{debug, error};

use crate::{
    Hex, Instruction, Part, PartType, Precursor, Rotation, Solution, StatField, Stats,
    version::{CURRENT_VERSION, SOME_FIRST_VERSION, SOME_OLD_VERSION},
};
use std::{
    collections::HashMap,
    io::{Read, Write},
};

type Result<T, E = &'static str> = std::result::Result<T, E>;

pub trait BinData<E = &'static str>: Sized {
    /// Fallible
    fn read<R: Read>(reader: &mut R) -> Result<Self, E>;
    /// Fallible
    fn write<W: Write>(&self, writer: &mut W) -> Result<(), E>;
}

impl<D1: BinData, D2: BinData> BinData for (D1, D2) {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let a = D1::read(reader)?;
        let b = D2::read(reader)?;
        Ok((a, b))
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.write(writer)?;
        self.1.write(writer)?;
        Ok(())
    }
}

impl BinData for u8 {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0; 1];
        reader
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read u8")?;
        Ok(buf[0])
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[*self]).map_err(|_| "Failed to write u8")
    }
}

impl BinData for bool {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let val = u8::read(reader)?;
        Ok(val != 0)
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer
            .write_all(&[*self as u8])
            .map_err(|_| "Failed to write bool")
    }
}

impl<D: BinData> BinData for Option<D> {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        if bool::read(reader)? {
            Ok(Some(D::read(reader)?))
        } else {
            Ok(None)
        }
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Some(data) => {
                true.write(writer)?;
                data.write(writer)
            }
            None => false.write(writer),
        }
    }
}

impl BinData for u32 {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0; 4];
        reader
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read u32")?;
        Ok(u32::from_le_bytes(buf))
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer
            .write_all(&self.to_le_bytes())
            .map_err(|_| "Failed to write u32")
    }
}

impl BinData for i32 {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0; 4];
        reader
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read i32")?;
        Ok(i32::from_le_bytes(buf))
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer
            .write_all(&self.to_le_bytes())
            .map_err(|_| "Failed to write i32")
    }
}

impl<D: BinData> BinData for Vec<D> {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let len = i32::read(reader)? as usize;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(D::read(reader)?);
        }
        Ok(vec)
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let len = self.len() as i32;
        len.write(writer)?;
        for item in self {
            item.write(writer)?;
        }
        Ok(())
    }
}

impl BinData for String {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let buf = Vec::<u8>::read(reader)?;
        Ok(String::from_utf8(buf).map_err(|_| "Failed to convert to String")?)
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let buf = self.bytes().collect::<Vec<u8>>();
        Ok(buf.write(writer)?)
    }
}

impl BinData for Part {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        debug!("=== Reading part ===");
        let part_type = PartType::read(reader)?;
        debug!("Part type: {:?}", part_type);
        let pos = Hex::<i32>::read(reader)?;
        debug!("Position: {:?}", pos);
        let rot = Rotation::read(reader)?;
        debug!("Rotation: {:?}", rot);
        let precursor = Option::<Precursor>::read(reader)?;
        debug!("Precursor: {:?}", precursor);
        // This is possibly the id of the beam
        let _int_1 = i32::read(reader)?;
        debug!("Int 1: {}", _int_1);
        let instructions = Vec::<Instruction>::read(reader)?;
        debug!("Instructions: {:?}", instructions);
        Ok(Self {
            part_type,
            pos,
            rot,
            precursor,
            instructions,
        })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        todo!()
    }
}

impl BinData for Option<Stats> {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let pairs: HashMap<i32, i32> = Vec::<(i32, i32)>::read(reader)?.into_iter().collect();
        Ok((|| {
            let cycles = *pairs.get(&StatField::Cycles.into())? as u32;
            let modules = *pairs.get(&StatField::Modules.into())? as u32;
            let symbols = *pairs.get(&StatField::Symbols.into())? as u32;
            Some(Stats {
                cycles,
                modules,
                symbols,
            })
        })())
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut pairs: Vec<(i32, i32)> = Vec::new();
        if let Some(stats) = self {
            pairs.push((StatField::Cycles.into(), stats.cycles as i32));
            pairs.push((StatField::Modules.into(), stats.modules as i32));
            pairs.push((StatField::Symbols.into(), stats.symbols as i32));
        }
        pairs.write(writer)
    }
}

impl BinData for Solution {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let version = i32::read(reader)?;
        debug!("Version: {}", version);

        if version > CURRENT_VERSION || version < SOME_OLD_VERSION {
            error!("Invalid version: {}", version);
            return Err("Invalid version");
        }

        let puzzle_id = i32::read(reader)?;
        debug!("Puzzle ID: {}", puzzle_id);

        let name = String::read(reader)?.trim().to_string();
        debug!("Name: {}", name);

        let _stats = Option::<Stats>::read(reader)?;
        debug!("Stats: {:?}", _stats);

        let parts = if version >= SOME_FIRST_VERSION {
            Vec::<Part>::read(reader)?
        } else {
            Vec::new()
        };

        Ok(Self {
            name,
            puzzle_id,
            parts,
        })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        todo!()
    }
}

impl<T: BinData> BinData for Hex<T> {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let a = T::read(reader)?;
        let b = T::read(reader)?;
        Ok(Self { a, b })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.a.write(writer)?;
        self.b.write(writer)?;
        Ok(())
    }
}

impl BinData for Rotation {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let val = i32::read(reader)?;
        Ok(val.into())
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let val: i32 = (*self).into();
        val.write(writer)
    }
}

impl BinData for Instruction {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let val = u8::read(reader)?;
        Ok(val.try_into().map_err(|err| {
            error!("Invalid instruction value: {}", val);
            err
        })?)
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let val: u8 = (*self).into();
        val.write(writer)
    }
}

impl BinData for PartType {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let val = i32::read(reader)?;
        Ok(val.try_into().map_err(|_| "Invalid part type")?)
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let val: i32 = (*self).into();
        val.write(writer)
    }
}

impl BinData for Precursor {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let val = i32::read(reader)?;
        Ok(val.try_into().map_err(|_| "Invalid precursor value")?)
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let val: i32 = (*self).into();
        val.write(writer)
    }
}
