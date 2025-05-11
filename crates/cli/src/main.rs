use std::{
    any::Any,
    fs::File,
    io::{Error as IoError, Read, Seek, Write, stdin, stdout},
    path::Path,
};

use anyhow::Result;
use cpiolib::{
    Header,
    ext::{ReadExt, WriteExt, WriteSeekPadExt},
};
use minibinrw::{MiniBinError, MiniBinRead, MiniBinWrite};

mod cli;

use cli::{Cli, Command, StripArgs, cli};

fn cpiostrip<W: Write + Seek>(
    input: &mut impl Read,
    output: &mut W,
    args: &StripArgs,
) -> Result<()> {
    let mut ino = 1;
    let mut _h_old;
    'main: loop {
        let mut h = match Header::m_read(input) {
            Ok(res) => res,
            Err(e) => {
                'emptychk: {
                    let MiniBinError::BadMagic(mag) = &e else {
                        break 'emptychk;
                    };
                    let magr = mag.as_inner();
                    let magt = magr as &dyn Any;
                    let magd = magt.downcast_ref::<[u8; 6]>();
                    let Some(data) = magd else {
                        break 'emptychk;
                    };
                    if data != &[0; 6] {
                        break 'emptychk;
                    }
                    break 'main;
                }
                return Err(e.into());
            }
        };
        if args.reset_ino {
            h.ino = ino;
            ino += 1;
        }
        if args.reset_mtime {
            h.mtime = 0;
        }
        h.m_write(output)?;
        if h.filesize != 0 {
            let size = h.filesize as usize;
            let mut buf = vec![0; size];
            input.read_exact(&mut buf)?;
            input.read_pad(size)?;
            output.write_all(&buf)?;
            output.write_pad(size)?;
        }
        if h.is_trailer() {
            break;
        }
        _h_old = h;
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
        E::Strip(args) => rscpio_strip(args),
    }
}
