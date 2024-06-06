use std::io::{BufRead, Cursor, Read};

use crate::error::ReadError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Endian {
    Little,
    Big,
}

pub(crate) fn read_uleb128(cursor: &mut Cursor<&[u8]>) -> Result<u8, ReadError> {
    let mut buf = [0u8; 1];
    match cursor.read(&mut buf) {
        Ok(1) => Ok(buf[0] & 0x7f),
        Ok(_) => Err(ReadError::Eof),
        Err(e) => Err(ReadError::Io(e)),
    }
}

pub(crate) fn read_uleb128_list(cursor: &mut Cursor<&[u8]>) -> Result<Box<[u8]>, ReadError> {
    let mut list = vec![];
    cursor.read_until(0, &mut list).map_err(ReadError::Io)?;
    if list.last() == Some(&0) {
        // remove null terminator
        list.pop();
        Ok(list.into_boxed_slice())
    } else {
        Err(ReadError::Eof)
    }
}

pub(crate) fn read_u32(cursor: &mut Cursor<&[u8]>, endian: Endian) -> Result<u32, ReadError> {
    let mut buf = [0u8; 4];
    match cursor.read(&mut buf) {
        Ok(4) => Ok(match endian {
            Endian::Little => u32::from_le_bytes(buf),
            Endian::Big => u32::from_be_bytes(buf),
        }),
        Ok(_) => Err(ReadError::Eof),
        Err(e) => Err(ReadError::Io(e)),
    }
}

pub(crate) fn read_string(cursor: &mut Cursor<&[u8]>) -> Result<String, ReadError> {
    let mut buf = vec![];
    cursor.read_until(0, &mut buf).map_err(ReadError::Io)?;
    if buf.last() == Some(&0) {
        // remove null terminator
        buf.pop();
        String::from_utf8(buf).map_err(ReadError::Utf8)
    } else {
        Err(ReadError::Eof)
    }
}
