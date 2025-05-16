use std::io::{Read, Write};

use anyhow::Result;
use cpiolib::{
    CpioEntry,
    ext::{CpioIterExt, WriteExt, WriteSeekPadExt},
};
use minibinrw::{BinWritable, MiniBinWrite};

use crate::cli::StripArgs;

fn cpio_strip_with_sort(
    input: &mut impl Read,
    output: &mut impl Write,
    args: &StripArgs,
) -> Result<()> {
    let mut entries = input
        .read_as_cpio_with_trailer()
        .collect::<Result<Vec<_>, _>>()?;
    let trailer = entries.pop();
    let mut ino = 1;
    entries.sort_by(|a, b| a.header.name.cmp(&b.header.name));
    for CpioEntry { header, .. } in &mut entries {
        if args.reset_ino {
            header.ino = ino;
            ino += 1;
        }
        if args.reset_mtime {
            header.mtime = 0;
        }
    }
    for entry in entries {
        entry.m_write_ne(output)?;
    }
    if let Some(trailer) = trailer {
        trailer.m_write_ne(output)?;
    };
    Ok(())
}

fn cpio_strip_without_sort(
    input: &mut impl Read,
    output: &mut impl Write,
    args: &StripArgs,
) -> Result<()> {
    let mut ino = 1;
    for entry in input.read_as_cpio_with_trailer() {
        let CpioEntry {
            mut header,
            contents,
        } = entry?;
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
    Ok(())
}

pub fn cpio_strip(
    input: &mut impl Read,
    output: &mut impl BinWritable,
    args: &StripArgs,
) -> Result<()> {
    if args.sort {
        cpio_strip_with_sort(input, output, args)?;
    } else {
        cpio_strip_without_sort(input, output, args)?;
    }
    output.write_padding(512, 0)?;
    Ok(())
}
