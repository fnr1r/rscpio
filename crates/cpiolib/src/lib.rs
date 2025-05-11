use std::{ffi::CString, io::Read};

use minibinrw::MiniBinRead;

pub mod binrw_impls;
pub mod ext;

pub const HEADER_LEN: usize = 110;

pub const MAGIC_NEWC: &[u8] = b"070701";
//pub const MAGIC_NUMBER_NEWCRC: &[u8] = b"070702";

pub const TRAILER_NAME: &str = "TRAILER!!!";
pub const TRAILER_SIZE: usize = HEADER_LEN + 10 + 4;

pub fn trailer_name_cstring() -> CString {
    let txt = CString::new(TRAILER_NAME);
    // SAFETY: TRAILER_NAME does not contain null
    unsafe { txt.unwrap_unchecked() }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub nlink: u32,
    pub mtime: u32,
    pub filesize: u32,
    pub devmajor: u32,
    pub devminor: u32,
    pub rdevmajor: u32,
    pub rdevminor: u32,
    pub _checksum: u32,
    pub name: CString,
}

impl Header {
    pub fn new_trailer(ino: u32) -> Self {
        let name = trailer_name_cstring();
        Header {
            ino,
            mode: 0,
            uid: 0,
            gid: 0,
            nlink: 1,
            mtime: 0,
            filesize: 0,
            devmajor: 0,
            devminor: 0,
            rdevmajor: 0,
            rdevminor: 0,
            _checksum: 0,
            name,
        }
    }
    pub fn is_trailer(&self) -> bool {
        self.name.as_bytes() == TRAILER_NAME.as_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct CpioEntry {
    pub header: Header,
    pub contents: Vec<u8>,
}

impl CpioEntry {
    pub fn is_trailer(&self) -> bool {
        self.header.is_trailer()
    }
}

#[derive(Debug)]
pub struct CpioIter<T: Read>(T, bool);

impl<T: Read> Iterator for CpioIter<T> {
    type Item = CpioEntry;
    fn next(&mut self) -> Option<Self::Item> {
        let Ok(entry) = CpioEntry::m_read_ne(&mut self.0) else {
            return None;
        };
        if entry.is_trailer() {
            return if self.1 { Some(entry) } else { None };
        }
        Some(entry)
    }
}
