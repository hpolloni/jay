use crate::constant_pool::{ConstantPool, Constant };
use byteorder::{BE, ReadBytesExt};
use int_enum::{IntEnum, IntEnumError};
use std::str;
use std::str::Utf8Error;
use std::io::{Error as IoError, BufRead};

#[repr(u8)]
#[derive(Clone, Debug, Copy, Eq, PartialEq, IntEnum)]
enum ConstantTag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    NameAndType = 12,
    MethodRef = 10,
    FieldRef = 9,
}

#[derive(Debug)]
pub struct ClassFile {
    constant_pool: ConstantPool,
    this_name: String,
    super_name: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    name: String,
    descriptor: String
}

impl ClassFile {
    pub fn parse(bytes: &mut &[u8]) -> Result<ClassFile, ParserError> {
        let magic= bytes.read_u32::<BE>()?;
        if magic != 0xCAFEBABE {
            return Err(ParserError::InvalidMagicHeader);
        }

        // TODO: we don't care about minor versions
        bytes.read_u16::<BE>()?;

        // TODO: we should support versions != 61 as bytecode should be backward compatible
        let major = bytes.read_u16::<BE>()?;
        if major != 61 {
            return Err(ParserError::UnsupportedVersion);
        }

        let cp = Self::parse_constant_pool(bytes)?;

        if ! cp.validate() {
            return Err(ParserError::InvalidConstantPool);
        }

        dbg!(&cp);

        // TODO: access flags
        bytes.read_u16::<BE>()?;

        let this_class = usize::from(bytes.read_u16::<BE>()? - 1);
        let super_class= usize::from(bytes.read_u16::<BE>()? - 1);

        // TODO: interfaces
        let interfaces_count = usize::from(bytes.read_u16::<BE>()?);
        dbg!(interfaces_count);
        bytes.consume(interfaces_count * 2);

        let fields_count = usize::from(bytes.read_u16::<BE>()?);
        dbg!(fields_count);

        let mut fields = vec!();

        for _ in 0..fields_count {
            /*
            u2             access_flags;
            u2             name_index;
            u2             descriptor_index;
            u2             attributes_count;
            attribute_info attributes[attributes_count];
            */
            // access_flags
            bytes.read_u16::<BE>()?;
            let name_index = usize::from(bytes.read_u16::<BE>()? - 1);
            let descriptor_index = usize::from(bytes.read_u16::<BE>()? - 1);
            let name = String::from(cp.get_utf8_at(name_index).unwrap());
            dbg!(name_index);
            dbg!(descriptor_index);

            let descriptor = String::from(cp.get_utf8_at(descriptor_index).unwrap());
            let attribs_count = bytes.read_u16::<BE>()?;
            dbg!(attribs_count);

            fields.push(Field { name, descriptor });
        }

        let this_name = String::from(cp.get_class_name(this_class));
        let super_name = String::from(cp.get_class_name(super_class));
        Ok(ClassFile{ constant_pool: cp, this_name, super_name, fields })
    }

    fn parse_constant_pool(bytes: &mut &[u8]) -> Result<ConstantPool, ParserError> {
        let mut cp = ConstantPool::new();
        let constant_pool_count = bytes.read_u16::<BE>()?;
        for _ in 1..constant_pool_count {
            let tag = ConstantTag::from_int(bytes.read_u8()?)?;
            dbg!(tag);
            match tag {
                ConstantTag::Class =>
                    cp.push(Constant::Class(usize::from(bytes.read_u16::<BE>()?) - 1)),

                ConstantTag::NameAndType =>
                    cp.push(Constant::NameAndType(usize::from(bytes.read_u16::<BE>()? - 1),
                                                  usize::from(bytes.read_u16::<BE>()? - 1))),

                ConstantTag::MethodRef =>
                    cp.push(Constant::MethodRef(usize::from(bytes.read_u16::<BE>()? - 1),
                                                usize::from(bytes.read_u16::<BE>()? - 1))),

                ConstantTag::FieldRef =>
                    cp.push(Constant::FieldRef(usize::from(bytes.read_u16::<BE>()? - 1),
                                               usize::from(bytes.read_u16::<BE>()? - 1))),

                ConstantTag::Utf8 => {
                    let length = usize::from(bytes.read_u16::<BE>()?);
                    cp.push(Constant::Utf8(str::from_utf8(&bytes[..length])?.to_string()));
                    bytes.consume(length);
                },

                ConstantTag::Integer => { todo!() },
                ConstantTag::Float => { todo!() },
                ConstantTag::Long => { todo!() },
                ConstantTag::Double => { todo!() },
            }
        }
        Ok(cp)
    }

    pub fn name(&self) -> &str {
        &self.this_name[..]
    }

    pub fn super_name(&self) -> &str {
        &self.super_name[..]
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ParserError {
    InvalidMagicHeader,
    ReadFailure,
    UnsupportedVersion,
    InvalidConstantTag,
    InvalidConstantPool,
    InvalidUtf8Encoding
}

impl From<IntEnumError<ConstantTag>> for ParserError {
    fn from(_: IntEnumError<ConstantTag>) -> Self {
        ParserError::InvalidConstantTag
    }
}

impl From<IoError> for ParserError {
    fn from(_: IoError) -> Self {
        ParserError::ReadFailure
    }
}

impl From<Utf8Error> for ParserError {
    fn from(_: Utf8Error) -> Self {
        ParserError::InvalidUtf8Encoding
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_magic() {
        let mut buf: &[u8] = &[0xCA, 0xCA, 0xBA, 0xBA];
        assert!(matches!(ClassFile::parse(&mut buf), Err(ParserError::InvalidMagicHeader)));
    }

    #[test]
    fn can_parse_version() {
        // Major version != 61
        let mut buf: &[u8] = &[0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00, 0x3f];
        assert!(matches!(ClassFile::parse(&mut buf), Err(ParserError::UnsupportedVersion)));
    }
}