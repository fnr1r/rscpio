use std::io::Read;

use anyhow::Result;
use cpiolib::{Header, ext::CpioIterExt};
use fstr::FStr as StackString;
use libc::{S_ISGID, S_ISUID, S_ISVTX};
use rustix::fs::FileType;
use static_slicing::{SliceWrapper, StaticIndex, StaticRangeIndex};

use crate::cli::{ListArgs, SharedArgs};

fn ftypelet(buf: &mut u8, filetype: FileType) {
    use FileType as E;
    match filetype {
        E::RegularFile => (),
        E::Directory => *buf = b'd',
        E::Symlink => *buf = b'l',
        E::Fifo => *buf = b'p',
        E::Socket => *buf = b's',
        E::CharacterDevice => *buf = b'c',
        E::BlockDevice => *buf = b'b',
        E::Unknown => *buf = b'?',
    }
}

fn rwx(buf: &mut [u8; 3], bits: u32) {
    if bits & 4 != 0 {
        buf[0] = b'r';
    }
    if bits & 2 != 0 {
        buf[1] = b'w';
    }
    if bits & 1 != 0 {
        buf[2] = b'x';
    }
}

fn setst(buf: &mut [u8; 11], header: &Header) {
    #[inline]
    fn cond_set_bit(bit: &mut u8, v1: u8, v2: u8) {
        if *bit != b'x' {
            *bit = v1;
        } else {
            *bit = v2;
        }
    }
    #[inline]
    fn cond_set_xid_bit(bit: &mut u8) {
        cond_set_bit(bit, b'S', b's');
    }
    if header.mode & S_ISUID != 0 {
        cond_set_xid_bit(&mut buf[3]);
    }
    if header.mode & S_ISGID != 0 {
        cond_set_xid_bit(&mut buf[6]);
    }
    if header.mode & S_ISVTX != 0 {
        cond_set_bit(&mut buf[9], b'T', b't');
    }
}

fn mode_string(buf: &mut [u8; 11], header: &Header) {
    let mut buf = SliceWrapper::new(buf);
    ftypelet(&mut buf[StaticIndex::<0>], header.file_type());
    rwx(
        &mut buf[StaticRangeIndex::<1, 3>],
        (header.mode & 0o700) >> 6,
    );
    rwx(
        &mut buf[StaticRangeIndex::<4, 3>],
        (header.mode & 0o070) >> 3,
    );
    rwx(&mut buf[StaticRangeIndex::<7, 3>], header.mode & 0o007);
    setst(&mut buf, header);
}

fn get_mode_string(header: &Header) -> StackString<11> {
    let mut buf = [b'-'; 11];
    mode_string(&mut buf, header);
    StackString::from_inner(buf).unwrap()
}

pub fn cpio_list(reader: &mut impl Read, args: &ListArgs, shared_args: &SharedArgs) -> Result<()> {
    let eol = if args.zero { '\0' } else { '\n' };
    for entry in reader.read_as_cpio() {
        let header = entry?.header;
        let txt = header.name.to_string_lossy();
        if shared_args.verbose {
            let modestr = get_mode_string(&header);
            print!("{}", modestr);
            print!("{:>3} ", header.nlink);
            print!("{:<8} {:<8} ", header.uid, header.gid);
            print!("{:>8} ", header.filesize);
            print!("{}", header.name.to_string_lossy());
            println!();
        } else {
            print!("{}{}", txt, eol);
        }
    }
    Ok(())
}
