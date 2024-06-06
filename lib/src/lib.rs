pub mod error;
pub mod globals;
pub mod read;
pub mod tag;

use std::io::{Cursor, Seek};

use error::{BuildAttrError, PublicAttrsError, ReadError, TagError};
use read::{read_string, read_u32, Endian};
use tag::{Scope, Tag};

pub struct BuildAttrs<'a> {
    data: &'a [u8],
    endian: Endian,
}

impl<'a> BuildAttrs<'a> {
    pub fn new(data: &'a [u8], endian: Endian) -> Result<Self, BuildAttrError> {
        if data.is_empty() {
            Err(BuildAttrError::NoData)
        } else {
            let attrs = Self { data, endian };
            let version = attrs.version();
            if version != b'A' {
                Err(BuildAttrError::IncompatibleVersion(version))
            } else {
                Ok(attrs)
            }
        }
    }

    pub fn version(&self) -> u8 {
        self.data[0]
    }

    pub fn subsections(&self) -> SubsectionIter {
        let data = &self.data[1..];
        SubsectionIter {
            data,
            cursor: Cursor::new(data),
            endian: self.endian,
        }
    }
}

pub struct SubsectionIter<'a> {
    data: &'a [u8],
    cursor: Cursor<&'a [u8]>,
    endian: Endian,
}

impl<'a> Iterator for SubsectionIter<'a> {
    type Item = Result<Subsection<'a>, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        let length = match read_u32(&mut self.cursor, self.endian) {
            Ok(length) => length,
            Err(ReadError::Eof) => return None,
            Err(e) => return Some(Err(e)),
        };
        let vendor_name = match read_string(&mut self.cursor) {
            Ok(vendor_name) => vendor_name,
            Err(ReadError::Eof) => return None,
            Err(e) => return Some(Err(e)),
        };
        let name_size = vendor_name.len() + 1;

        let pos = self.cursor.position() as usize;
        let data = &self.data[pos..pos + length as usize - name_size - 4];
        if let Err(e) = self.cursor.seek(std::io::SeekFrom::Current(data.len() as i64)) {
            Some(Err(ReadError::Io(e)))
        } else {
            Some(Ok(Subsection {
                data,
                endian: self.endian,
                vendor_name,
            }))
        }
    }
}

pub struct Subsection<'a> {
    data: &'a [u8],
    endian: Endian,
    vendor_name: String,
}

impl<'a> Subsection<'a> {
    pub fn is_aeabi(&self) -> bool {
        self.vendor_name == "aeabi"
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    pub fn endian(&self) -> Endian {
        self.endian
    }

    pub fn vendor_name(&self) -> &str {
        self.vendor_name.as_str()
    }
}

impl<'a> Subsection<'a> {
    pub fn public_attributes(self) -> Result<PublicAttrs<'a>, PublicAttrsError> {
        if self.is_aeabi() {
            Ok(PublicAttrs {
                data: self.data,
                endian: self.endian,
            })
        } else {
            Err(PublicAttrsError::InvalidName(self.vendor_name))
        }
    }
}

pub struct PublicAttrs<'a> {
    data: &'a [u8],
    endian: Endian,
}

impl<'a> PublicAttrs<'a> {
    pub fn attributes(&self) -> Result<PublicAttrIter, TagError> {
        let mut cursor = Cursor::new(self.data);
        let first_tag = Tag::read(&mut cursor, self.endian)?;
        let scope = Scope::new(first_tag)?;
        Ok(PublicAttrIter {
            cursor,
            endian: self.endian,
            scope,
        })
    }
}

pub struct PublicAttrIter<'a> {
    cursor: Cursor<&'a [u8]>,
    endian: Endian,
    scope: Scope,
}

impl<'a> Iterator for PublicAttrIter<'a> {
    type Item = Result<Tag, TagError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let tag = match Tag::read(&mut self.cursor, self.endian) {
                Ok(tag) => tag,
                Err(TagError::Read(ReadError::Eof)) => return None,
                Err(e) => return Some(Err(e)),
            };
            match tag {
                Tag::File { size: _ } | Tag::Section { size: _, sections: _ } | Tag::Symbol { size: _, symbols: _ } => {
                    // update scope
                    match Scope::new(tag) {
                        Ok(scope) => self.scope = scope,
                        Err(e) => return Some(Err(e)),
                    }
                }
                _ => return Some(Ok(tag)),
            }
        }
    }
}
