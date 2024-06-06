use std::{io, string::FromUtf8Error};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildAttrError {
    #[error("no data")]
    NoData,
    #[error("incompatible format version")]
    IncompatibleVersion(u8),
}

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("IO error")]
    Io(io::Error),
    #[error("UTF8 error")]
    Utf8(FromUtf8Error),
    #[error("no more data (EOF reached)")]
    Eof,
}

#[derive(Error, Debug)]
pub enum PublicAttrsError {
    #[error("invalid subsection name, should be 'aeabi'")]
    InvalidName(String),
}

#[derive(Error, Debug)]
pub enum TagError {
    #[error("incompatible tag value")]
    IncompatibleTagValue(u8),
    #[error("read error")]
    Read(ReadError),
    #[error("invalid scope tag")]
    InvalidScopeTag,
}
