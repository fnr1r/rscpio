use std::{
    fs::File,
    io::{Result, stdin, stdout},
    path::PathBuf,
};

/*use minibinrw::{BinReadable, BinWritable};

#[derive(Debug)]
struct PartialCursor<T> {
    pos: u64,
    inner: T,
}

impl<T> PartialCursor<T> {
    fn new(inner: T) -> Self {
        Self { inner, pos: 0 }
    }
}

impl<T: Read> Read for PartialCursor<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.inner.read(buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

fn part_cur_err() -> Error {
    Error::new(
        ErrorKind::NotSeekable,
        "PartialCursor does not support seeking",
    )
}

impl<T> Seek for PartialCursor<T> {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> Result<u64> {
        Err(part_cur_err())
    }
    fn rewind(&mut self) -> Result<()> {
        Err(part_cur_err())
    }
    fn seek_relative(&mut self, _offset: i64) -> Result<()> {
        Err(part_cur_err())
    }
    fn stream_position(&mut self) -> Result<u64> {
        Ok(self.pos)
    }
}*/

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
