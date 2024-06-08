use std::{
    io::{Cursor, Read},
    str::from_utf8,
};

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

pub(crate) fn read_u8(cursor: &mut Cursor<&[u8]>) -> Result<u8, ReadError> {
    let mut buf = [0u8; 1];
    match cursor.read(&mut buf) {
        Ok(1) => Ok(buf[0]),
        Ok(_) => Err(ReadError::Eof),
        Err(e) => Err(ReadError::Io(e)),
    }
}

pub(crate) fn read_uleb128_list<'a>(cursor: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], ReadError> {
    let pos = cursor.position() as usize;
    let data = cursor.get_mut();
    let data = &data[pos..];
    let len = data.iter().position(|x| *x == 0).unwrap_or(data.len());
    let list = &data[..len];
    cursor.set_position(pos as u64 + len as u64 + 1);
    Ok(list)
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

pub(crate) fn read_string<'a>(cursor: &mut Cursor<&'a [u8]>) -> Result<&'a str, ReadError> {
    let pos = cursor.position() as usize;
    let data = cursor.get_mut();
    let data = &data[pos..];
    let len = data.iter().position(|x| *x == 0).unwrap_or(data.len());
    let list = &data[..len];
    cursor.set_position(pos as u64 + len as u64 + 1);
    from_utf8(list).map_err(ReadError::Utf8)
}
