use std::ffi::CString;

pub mod binrw_impls;
pub mod ext;

const HEADER_LEN: usize = 110;

pub const MAGIC_NEWC: &[u8] = b"070701";
//pub const MAGIC_NUMBER_NEWCRC: &[u8] = b"070702";

pub const TRAILER_NAME: &str = "TRAILER!!!";

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
