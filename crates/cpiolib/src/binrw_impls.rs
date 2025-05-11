use std::{
    ffi::CString,
    io::{Read, Write},
};

use easy_ext::ext;
use minibinrw::{
    BadMagicError, CustomError, MiniBinError, MiniBinRead, MiniBinWrite, Result,
    binrw::{Endian, Error as BinError},
    impl_binread_with_mini, impl_binwrite_with_mini,
};

use crate::{
    CpioEntry, HEADER_LEN, Header, MAGIC_NEWC,
    ext::{ReadExt, WriteExt},
};

#[ext]
impl<T> Option<T> {
    fn replace_ref(&mut self, value: T) -> &T {
        self.replace(value);
        // SAFETY: We just replaced the value, so it's Some
        unsafe { self.as_ref().unwrap_unchecked() }
    }
}

#[ext]
impl MiniBinError {
    fn into_binrw(self, pos: u64) -> BinError {
        match self {
            Self::BadMagic(magic) => BinError::BadMagic {
                pos,
                found: magic.into_inner(),
            },
            Self::Custom(e) => BinError::Custom {
                pos,
                err: e.into_inner(),
            },
            Self::IoError(e) => e.into(),
        }
    }
}

impl MiniBinRead for Header {
    fn m_read_options(reader: &mut impl Read, _endian: Endian) -> Result<Self> {
        let mut magic = [0; 6];
        reader.read_exact(&mut magic)?;
        if magic != MAGIC_NEWC {
            return Err(BadMagicError::new(magic).into());
        }
        let ino = reader.read_u32_hex()?;
        let mode = reader.read_u32_hex()?;
        let uid = reader.read_u32_hex()?;
        let gid = reader.read_u32_hex()?;
        let nlink = reader.read_u32_hex()?;
        let mtime = reader.read_u32_hex()?;
        let filesize = reader.read_u32_hex()?;
        let devmajor = reader.read_u32_hex()?;
        let devminor = reader.read_u32_hex()?;
        let rdevmajor = reader.read_u32_hex()?;
        let rdevminor = reader.read_u32_hex()?;
        let namesize = reader.read_u32_hex()?;
        let _checksum = reader.read_u32_hex()?;
        let mut name = vec![0; namesize as usize];
        reader.read_exact(&mut name)?;
        let name = CString::from_vec_with_nul(name).map_err(CustomError::new)?;
        reader.read_pad(HEADER_LEN + namesize as usize)?;
        Ok(Header {
            ino,
            mode,
            uid,
            gid,
            nlink,
            mtime,
            filesize,
            devmajor,
            devminor,
            rdevmajor,
            rdevminor,
            _checksum,
            name,
        })
    }
}

impl_binread_with_mini!(Header);

impl MiniBinWrite for Header {
    fn m_write_options(&self, writer: &mut impl Write, _endian: Endian) -> Result<()> {
        let name = self.name.as_bytes_with_nul();
        writer.write_all(MAGIC_NEWC)?;
        writer.write_u32_hex(self.ino)?;
        writer.write_u32_hex(self.mode)?;
        writer.write_u32_hex(self.uid)?;
        writer.write_u32_hex(self.gid)?;
        writer.write_u32_hex(self.nlink)?;
        writer.write_u32_hex(self.mtime)?;
        writer.write_u32_hex(self.filesize)?;
        writer.write_u32_hex(self.devmajor)?;
        writer.write_u32_hex(self.devminor)?;
        writer.write_u32_hex(self.rdevmajor)?;
        writer.write_u32_hex(self.rdevminor)?;
        writer.write_u32_hex(name.len() as u32)?;
        writer.write_u32_hex(self._checksum)?;
        writer.write_all(name)?;
        writer.write_pad(HEADER_LEN + name.len())?;
        Ok(())
    }
}

impl_binwrite_with_mini!(Header);

impl MiniBinRead for CpioEntry {
    fn m_read_options(reader: &mut impl Read, endian: Endian) -> Result<Self> {
        let header = Header::m_read_options(reader, endian)?;
        let mut contents = vec![0; header.filesize as usize];
        reader.read_exact(&mut contents)?;
        reader.read_pad(header.filesize as usize)?;
        Ok(Self { header, contents })
    }
}

impl_binread_with_mini!(CpioEntry);

impl MiniBinWrite for CpioEntry {
    fn m_write_options(&self, writer: &mut impl Write, endian: Endian) -> Result<()> {
        let mut hcopy = None;
        let href = if self.contents.len() == self.header.filesize as usize {
            &self.header
        } else {
            let mut header = Box::new(self.header.clone());
            header.filesize = self.contents.len() as u32;
            header._checksum = 0;
            hcopy.replace_ref(header)
        };
        href.m_write_options(writer, endian)?;
        writer.write_all(&self.contents)?;
        writer.write_pad(self.contents.len())?;
        Ok(())
    }
}

impl_binwrite_with_mini!(CpioEntry);
