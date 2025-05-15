use std::{ffi::CString, io::BufRead, path::Path};

use anyhow::Result;
use camino::Utf8Path;
use cpiolib::{CpioEntry, Header, ext::WriteSeekPadExt};
use easy_ext::ext;
use minibinrw::{BinWritable, MiniBinWrite};
use rustix::fs::{Dev, FileType, Stat, lstat, major, minor, stat};

use crate::cli::{CreateArgs, SharedArgs};

#[derive(Debug)]
struct LineIter<T: BufRead> {
    buf: String,
    inner: T,
}

impl<T: BufRead> Iterator for LineIter<T> {
    type Item = Box<str>;
    fn next(&mut self) -> Option<Self::Item> {
        let buf = &mut self.buf;
        buf.clear();
        self.inner.read_line(buf).ok()?;
        Some(buf.clone().into_boxed_str())
    }
}

#[ext(ReadLineExt)]
impl<T: BufRead> T {
    fn read_linef(&mut self) -> impl Iterator<Item = Box<str>> {
        LineIter {
            buf: String::new(),
            inner: self,
        }
    }
}

fn devsplit(dev: Dev) -> (u32, u32) {
    (major(dev), minor(dev))
}

fn stat_to_cpio(name: &Utf8Path, stat: &Stat) -> Result<Header, std::ffi::NulError> {
    let name = name.as_str().as_bytes();
    let name = CString::new(name)?;
    let (devmajor, devminor) = devsplit(stat.st_dev);
    let (rdevmajor, rdevminor) = devsplit(stat.st_rdev);
    Ok(Header {
        ino: stat.st_ino as u32,
        mode: stat.st_mode,
        uid: stat.st_uid,
        gid: stat.st_gid,
        nlink: stat.st_nlink as u32,
        mtime: stat.st_mtime as u32,
        filesize: 0,
        devmajor,
        devminor,
        rdevmajor,
        rdevminor,
        _checksum: 0,
        name,
    })
}

pub fn cpio_create(
    input: &mut impl BufRead,
    output: &mut impl BinWritable,
    args: &CreateArgs,
    shared_args: &SharedArgs,
) -> Result<()> {
    let stat_impl = if args.input_shared.dereference {
        |p: &Path| stat(p)
    } else {
        |p: &Path| lstat(p)
    };
    for line in input.read_linef() {
        if line.is_empty() {
            break;
        }
        let linen = line.strip_suffix("\n").unwrap();
        if linen.is_empty() {
            continue;
        }
        let filename = Utf8Path::new(linen);
        let stat = stat_impl(filename.as_std_path())?;
        let mut header = stat_to_cpio(filename, &stat)?;
        use FileType as E;
        let contents = match header.file_type() {
            E::RegularFile => Some(std::fs::read(filename)?),
            E::Symlink => {
                let target = std::fs::read_link(filename)?;
                Some(target.into_os_string().into_encoded_bytes())
            }
            _ => None,
        }
        .unwrap_or_default();
        header.filesize = contents.len() as u32;
        let entry = CpioEntry { header, contents };
        entry.m_write_ne(output)?;
        if shared_args.print_dot {
            print!(".");
        }
        if shared_args.verbose {
            println!("{}", filename);
        }
    }
    Header::new_trailer(0).m_write_ne(output)?;
    output.write_padding(512, 0)?;
    Ok(())
}
