use std::{io, str::Utf8Error};

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
    Utf8(Utf8Error),
    #[error("no more data (EOF reached)")]
    Eof,
}

#[derive(Error, Debug)]
pub enum PublicAttrsError {
    #[error("invalid subsection name, should be 'aeabi'")]
    InvalidName(String),
    #[error("tag error")]
    Tag(TagError),
    #[error("no tags")]
    NoTags,
    #[error("expected first tag to be a file tag")]
    NoFileTag,
    #[error("expected to be in section scope")]
    NoSectionScope,
}

#[derive(Error, Debug)]
pub enum TagError {
    #[error("incompatible tag value")]
    IncompatibleTagValue(u8),
    #[error("read error")]
    Read(ReadError),
    #[error("invalid scope tag")]
    InvalidScopeTag,
    #[error("expected null")]
    ExpectedNull,
}
