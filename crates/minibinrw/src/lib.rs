//! binrw, but without `Seek` requirement
//!
//! and with no derive macros

use std::io::{Read, Seek, Write};

use trait_set::trait_set;

mod error;
mod macros;

pub use error::{BadMagicError, CustomError, MiniBinError, Result};

pub use binrw;

trait_set! {
    pub trait BinReadable = Read + Seek;
    pub trait BinWritable = Write + Seek;
}

pub trait MiniBinRead: Sized {
    fn m_read(reader: &mut impl Read) -> Result<Self>;
}

pub trait MiniBinWrite: Sized {
    fn m_write(&self, writer: &mut impl Write) -> Result<()>;
}
