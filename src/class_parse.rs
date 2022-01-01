use byteorder::{BE, ReadBytesExt};
use std::io;


pub struct ClassFile {

}

#[derive(Debug)]
pub enum ParserError {
    InvalidMagicHeader,
    ReadFailure,
    UnsupportedVersion
}

impl From<io::Error> for ParserError {
    fn from(_: io::Error) -> Self {
        ParserError::ReadFailure
    }
}

pub fn parse<R: io::Read>(mut reader: R) -> std::result::Result<ClassFile, ParserError> {
    let magic= reader.read_u32::<BE>()?;
    if magic != 0xCAFEBABE {
        return Err(ParserError::InvalidMagicHeader);
    }

    // TODO: we don't care about minor versions
    reader.read_u16::<BE>()?;

    // TODO: we should support versions != 61
    let major = reader.read_u16::<BE>()?;
    if major != 61 {
        return Err(ParserError::UnsupportedVersion);
    }

    Ok(ClassFile{})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_magic() {
        let mut buf: &[u8] = &[0xCA, 0xCA, 0xBA, 0xBA];
        assert!(matches!(parse(&mut buf), Err(ParserError::InvalidMagicHeader)));

        buf = &[0xCA, 0xFE, 0xBA, 0xBE];
        assert!(matches!(parse(&mut buf), Err(ParserError::ReadFailure)));
    }

    #[test]
    fn can_parse_version() {
        let mut buf: &[u8] = &[0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00, 0x3d];
        assert!(parse(&mut buf).is_ok());

        // Unexpected end of stream
        buf = &[0xCA, 0xFE, 0xBA, 0xBE, 0x00];
        assert!(matches!(parse(&mut buf), Err(ParserError::ReadFailure)));

        buf = &[0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00];
        assert!(matches!(parse(&mut buf), Err(ParserError::ReadFailure)));

        // Major version != 61
        buf = &[0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00, 0x3f];
        assert!(matches!(parse(&mut buf), Err(ParserError::UnsupportedVersion)));
    }
}