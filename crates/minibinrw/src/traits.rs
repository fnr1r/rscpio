use std::io::{Read, Seek, Write};

use binrw::Endian;
use trait_set::trait_set;

use crate::Result;

trait_set! {
    pub trait BinReadable = Read + Seek;
    pub trait BinWritable = Write + Seek;
}

pub trait MiniBinRead: Sized {
    fn m_read_options(reader: &mut impl Read, endian: Endian) -> Result<Self>;
    #[inline]
    fn m_read_be(reader: &mut impl Read) -> Result<Self> {
        Self::m_read_options(reader, Endian::Big)
    }
    #[inline]
    fn m_read_le(reader: &mut impl Read) -> Result<Self> {
        Self::m_read_options(reader, Endian::Little)
    }
    #[inline]
    fn m_read_ne(reader: &mut impl Read) -> Result<Self> {
        Self::m_read_options(reader, Endian::NATIVE)
    }
}

pub trait MiniBinWrite: Sized {
    fn m_write_options(&self, writer: &mut impl Write, endian: Endian) -> Result<()>;
    #[inline]
    fn m_write_be(&self, writer: &mut impl Write) -> Result<()> {
        self.m_write_options(writer, Endian::Big)
    }
    #[inline]
    fn m_write_le(&self, writer: &mut impl Write) -> Result<()> {
        self.m_write_options(writer, Endian::Little)
    }
    #[inline]
    fn m_write_ne(&self, writer: &mut impl Write) -> Result<()> {
        self.m_write_options(writer, Endian::NATIVE)
    }
}
