use std::io::{Read, Write};

use anyhow::Result;
use cpiolib::{
    CpioEntry, Header,
    ext::{CpioIterExt, WriteExt, WriteSeekPadExt},
};
use minibinrw::{BinWritable, MiniBinWrite};

use crate::cli::{SharedArgs, StripArgs};

fn cpio_strip_entry(
    header: &mut Header,
    args: &StripArgs,
    shared_args: &SharedArgs,
    ino: &mut u32,
) {
    if shared_args.verbose {
        eprintln!("{}", header.name.to_string_lossy());
    } else if shared_args.print_dot {
        eprintln!(".");
    }
    if args.reset_ino {
        if shared_args.verbose {
            eprintln!("  ino: {} -> {}", header.ino, ino);
        }
        header.ino = *ino;
        *ino += 1;
    }
    if args.reset_mtime {
        if shared_args.verbose {
            eprintln!("  mtime: {} -> NULL", header.mtime);
        }
        header.mtime = 0;
    }
}

fn cpio_strip_with_sort(
    input: &mut impl Read,
    output: &mut impl Write,
    args: &StripArgs,
    shared_args: &SharedArgs,
    ino: &mut u32,
) -> Result<()> {
    let mut entries = input
        .read_as_cpio_with_trailer()
        .collect::<Result<Vec<_>, _>>()?;
    let trailer = entries.pop();
    entries.sort_by(|a, b| a.header.name.cmp(&b.header.name));
    for CpioEntry { header, .. } in &mut entries {
        cpio_strip_entry(header, args, shared_args, ino);
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
    shared_args: &SharedArgs,
    ino: &mut u32,
) -> Result<()> {
    for entry in input.read_as_cpio_with_trailer() {
        let CpioEntry {
            mut header,
            contents,
        } = entry?;
        if !header.is_trailer() {
            cpio_strip_entry(&mut header, args, shared_args, ino);
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
    shared_args: &SharedArgs,
) -> Result<()> {
    let mut ino = 1;
    if args.sort {
        cpio_strip_with_sort(input, output, args, shared_args, &mut ino)?;
    } else {
        cpio_strip_without_sort(input, output, args, shared_args, &mut ino)?;
    }
    output.write_padding(512, 0)?;
    Ok(())
}
