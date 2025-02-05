use core::str::from_utf8;

use crate::error::ReadError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Endian {
    Little,
    Big,
}

pub(crate) struct Cursor<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) pos: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub(crate) fn read(&mut self, buf: &mut [u8]) -> usize {
        let data = &self.data[self.pos..];
        let len = buf.len().min(data.len());
        buf[..len].copy_from_slice(&data[..len]);
        self.pos += len;
        len
    }

    pub(crate) fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ReadError> {
        let len = self.read(buf);
        if len == buf.len() {
            Ok(())
        } else {
            Err(ReadError::Eof)
        }
    }

    pub(crate) fn position(&self) -> usize {
        self.pos
    }

    pub(crate) fn set_position(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub(crate) fn get_ref(&self) -> &'a [u8] {
        &self.data
    }

    pub(crate) fn remaining(&self) -> &'a [u8] {
        &self.data[self.pos..]
    }
}

pub(crate) fn read_uleb128(cursor: &mut Cursor) -> Result<u8, ReadError> {
    let mut buf = [0u8; 1];
    cursor.read_exact(&mut buf)?;
    Ok(buf[0] & 0x7f)
}

pub(crate) fn read_u8(cursor: &mut Cursor) -> Result<u8, ReadError> {
    let mut buf = [0u8; 1];
    cursor.read_exact(&mut buf)?;
    Ok(buf[0])
}

pub(crate) fn read_uleb128_list<'a>(cursor: &mut Cursor<'a>) -> Result<&'a [u8], ReadError> {
    let data = cursor.remaining();
    let len = data.iter().position(|x| *x == 0).unwrap_or(data.len());
    cursor.set_position(cursor.position() + len + 1);
    Ok(&data[..len])
}

pub(crate) fn read_u32(cursor: &mut Cursor, endian: Endian) -> Result<u32, ReadError> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf)?;
    Ok(match endian {
        Endian::Little => u32::from_le_bytes(buf),
        Endian::Big => u32::from_be_bytes(buf),
    })
}

pub(crate) fn read_string<'a>(cursor: &mut Cursor<'a>) -> Result<&'a str, ReadError> {
    let data = cursor.remaining();
    let len = data.iter().position(|x| *x == 0).unwrap_or(data.len());
    cursor.set_position(cursor.position() + len + 1);
    from_utf8(&data[..len]).map_err(ReadError::Utf8)
}
