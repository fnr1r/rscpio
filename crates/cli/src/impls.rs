use std::io::{BufReader, BufWriter, Read};

use anyhow::Result;
use cpiolib::{
    CpioEntry,
    ext::{CpioIterExt, WriteExt, WriteSeekPadExt},
};
use minibinrw::{BinWritable, MiniBinWrite};

use crate::{
    cli::{ListArgs, StripArgs},
    termio::{input_file_or_stdin, output_file_or_stdout},
};

fn cpio_list(reader: &mut impl Read, args: &ListArgs) -> Result<()> {
    let eol = if args.zero { '\0' } else { '\n' };
    for entry in reader.read_as_cpio() {
        let txt = entry.header.name.to_string_lossy();
        print!("{}{}", txt, eol);
    }
    Ok(())
}

fn cpio_strip_with_sort(
    input: &mut impl Read,
    output: &mut impl BinWritable,
    args: &StripArgs,
) -> Result<()> {
    let mut entries = input.read_as_cpio_with_trailer().collect::<Vec<_>>();
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

fn cpio_strip_without_sort<R: Read, W: BinWritable>(
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
    Ok(())
}

pub fn rscpio_list(args: ListArgs) -> Result<()> {
    let mut input_ref = BufReader::new(input_file_or_stdin(&args.input)?);
    cpio_list(&mut input_ref, &args)
}

pub fn rscpio_strip(args: StripArgs) -> Result<()> {
    let mut input_ref = BufReader::new(input_file_or_stdin(&args.input)?);
    let mut output_ref = BufWriter::new(output_file_or_stdout(&args.output)?);
    if args.sort {
        cpio_strip_with_sort(&mut input_ref, &mut output_ref, &args)?;
    } else {
        cpio_strip_without_sort(&mut input_ref, &mut output_ref, &args)?;
    }
    output_ref.write_padding(512, 0)?;
    Ok(())
}
