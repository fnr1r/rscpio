use std::{
    fs::File,
    io::{Error as IoError, Read, stdin, stdout},
    path::Path,
};

use anyhow::Result;
use cpiolib::{
    CpioEntry,
    ext::{CpioIterExt, WriteExt, WriteSeekPadExt},
};
use minibinrw::{BinWritable, MiniBinWrite};

mod cli;

use cli::{Cli, Command, ListArgs, StripArgs, cli};

fn cpio_list(reader: &mut impl Read, args: &ListArgs) -> Result<()> {
    let eol = if args.zero { '\0' } else { '\n' };
    for entry in reader.read_as_cpio() {
        let txt = entry.header.name.to_string_lossy();
        print!("{}{}", txt, eol);
    }
    Ok(())
}

fn cpiostrip<R: Read, W: BinWritable>(
    input: &mut R,
    output: &mut W,
    args: &StripArgs,
) -> Result<()> {
    let mut ino = 1;
    for CpioEntry {
        mut header,
        contents,
    } in input.read_as_cpio_with_trailer()
    {
        if args.reset_ino && !header.is_trailer() {
            header.ino = ino;
            ino += 1;
        }
        if args.reset_mtime {
            header.mtime = 0;
        }
        header.m_write_ne(output)?;
        let size = header.filesize as usize;
        if size != 0 {
            output.write_all(&contents)?;
            output.write_pad(size)?;
        }
    }
    output.write_padding(512, 0)?;
    Ok(())
}

/*fn cpiostrip(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut ino = 0;
    loop {
        let reader = NewcReader::new(input)?;
        let entry = reader.entry();
        if entry.is_trailer() {
            break;
        }
        let eb = NewcBuilder::new(entry.name())
            .ino(ino)
            .mode(entry.mode())
            .uid(entry.uid())
            .gid(entry.gid())
            .nlink(entry.nlink())
            .mtime(0)
            .dev_major(entry.dev_major())
            .dev_minor(entry.dev_minor())
            .rdev_major(entry.rdev_major())
            .rdev_minor(entry.rdev_minor());
        eb.write(&mut output, entry.file_size());
        output.write(reader.)
        let mut input = reader.finish()?;
    }
    Ok(())
}*/

fn open_file_or_stdin(path: Option<&Path>) -> Result<File, IoError> {
    Ok(if let Some(path) = path {
        File::open(path)?
    } else {
        use std::os::unix::io::{AsRawFd, FromRawFd};
        unsafe { File::from_raw_fd(stdin().as_raw_fd()) }
    })
}

fn open_file_or_stdout(path: Option<&Path>) -> Result<File, IoError> {
    Ok(if let Some(path) = path {
        File::create(path)?
    } else {
        use std::os::unix::io::{AsRawFd, FromRawFd};
        unsafe { File::from_raw_fd(stdout().as_raw_fd()) }
    })
}

fn rscpio_list(args: ListArgs) -> Result<()> {
    let input_ref = args.input.as_ref().map(|e| e.as_ref());
    let mut input_ref = open_file_or_stdin(input_ref)?;
    cpio_list(&mut input_ref, &args)
}

fn rscpio_strip(args: StripArgs) -> Result<()> {
    let input_ref = args.input.as_ref().map(|e| e.as_ref());
    let output_ref = args.output.as_ref().map(|e| e.as_ref());
    let mut input_ref = open_file_or_stdin(input_ref)?;
    let mut output_ref = open_file_or_stdout(output_ref)?;
    cpiostrip(&mut input_ref, &mut output_ref, &args)?;
    Ok(())
}

fn main() -> Result<()> {
    //println!("Hello, world!");
    let Cli { command } = cli();
    use Command as E;
    match command {
        E::List(args) => rscpio_list(args),
        E::Strip(args) => rscpio_strip(args),
    }
}
