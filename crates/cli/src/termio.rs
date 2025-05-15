use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write, stdin, stdout},
};

use camino::Utf8PathBuf as PathBuf;

fn part_cur_err() -> Error {
    Error::new(
        ErrorKind::NotSeekable,
        "PartialCursor does not support seeking",
    )
}

#[derive(Debug)]
pub struct PartialCursor<T> {
    pos: u64,
    inner: T,
}

impl<T> PartialCursor<T> {
    pub fn new(inner: T) -> Self {
        Self { inner, pos: 0 }
    }
}

impl<T> Seek for PartialCursor<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let SeekFrom::Current(pos) = pos else {
            return Err(part_cur_err());
        };
        if pos == 0 {
            Ok(self.pos)
        } else {
            Err(part_cur_err())
        }
    }
}

impl<T: Read> Read for PartialCursor<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.inner.read(buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl<T: Write> Write for PartialCursor<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.inner.write(buf)?;
        self.pos += n as u64;
        Ok(n)
    }
    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> Result<usize> {
        let n = self.inner.write_vectored(bufs)?;
        self.pos += n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

fn stdin_file() -> File {
    use std::os::unix::io::{AsRawFd, FromRawFd};
    unsafe { File::from_raw_fd(stdin().as_raw_fd()) }
}

fn stdout_file() -> File {
    use std::os::unix::io::{AsRawFd, FromRawFd};
    unsafe { File::from_raw_fd(stdout().as_raw_fd()) }
}

pub fn input_file_or_stdin(path: &Option<PathBuf>) -> Result<File> {
    let path = path.as_ref();
    if let Some(path) = path.map(|e| e.as_path()) {
        File::open(path)
    } else {
        Ok(stdin_file())
    }
}

pub fn output_file_or_stdout(path: &Option<PathBuf>) -> Result<File> {
    let path = path.as_ref();
    if let Some(path) = path.map(|e| e.as_path()) {
        File::create(path)
    } else {
        Ok(stdout_file())
    }
}
