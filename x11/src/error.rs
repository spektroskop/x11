use std::io::Read;

#[derive(Debug)]
pub enum Error {
    Request,
    Value(u32),
    Window(u32),
    Pixmap(u32),
    Atom(u32),
    Cursor(u32),
    Font(u32),
    Match,
    Drawable(u32),
    Access,
    Alloc,
    Colormap(u32),
    GContext(u32),
    IDChoice(u32),
    Name,
    Length,
    Implementation,
    Unknown(u8),
}

impl Error {
    pub fn read<T: Read>(r: &mut T) -> anyhow::Result<Self> {
        use byteorder::{BigEndian, ReadBytesExt};

        let code = r.read_u8()?;
        let _sequence_number = r.read_u16::<BigEndian>()?;

        let mut buf: [u8; 4] = [0; 4];
        r.read_exact(&mut buf)?;
        let mut cursor = std::io::Cursor::new(buf);

        let _major_opcode = r.read_u16::<BigEndian>()?;
        let _minor_opcode = r.read_u8()?;

        match code {
            1 => Ok(Error::Request),
            2 => Ok(Error::Value(cursor.read_u32::<BigEndian>()?)),
            3 => Ok(Error::Window(cursor.read_u32::<BigEndian>()?)),
            4 => Ok(Error::Pixmap(cursor.read_u32::<BigEndian>()?)),
            5 => Ok(Error::Atom(cursor.read_u32::<BigEndian>()?)),
            6 => Ok(Error::Cursor(cursor.read_u32::<BigEndian>()?)),
            7 => Ok(Error::Font(cursor.read_u32::<BigEndian>()?)),
            8 => Ok(Error::Match),
            9 => Ok(Error::Drawable(cursor.read_u32::<BigEndian>()?)),
            10 => Ok(Error::Access),
            11 => Ok(Error::Alloc),
            12 => Ok(Error::Colormap(cursor.read_u32::<BigEndian>()?)),
            13 => Ok(Error::GContext(cursor.read_u32::<BigEndian>()?)),
            14 => Ok(Error::IDChoice(cursor.read_u32::<BigEndian>()?)),
            15 => Ok(Error::Name),
            16 => Ok(Error::Length),
            17 => Ok(Error::Implementation),

            code => Ok(Error::Unknown(code)),
        }
    }
}
