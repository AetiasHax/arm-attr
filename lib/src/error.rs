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
    #[error("index out of bounds")]
    OutOfBounds,
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
    #[error("duplicate file tag")]
    DuplicateFileTag,
    #[error("expected to be in file scope")]
    NotFileScope,
    #[error("expected to be in section scope")]
    NotSectionScope,
    #[error("enclosed scope ends before parent scope")]
    ScopeEndsBeforeParent,
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
    #[error("nested scope tag")]
    NestedScopeTag,
}
