use std::io::{BufReader, BufWriter};

use anyhow::Result;

use crate::{
    cli::{ListArgs, StripArgs},
    termio::{PartialCursor, input_file_or_stdin, output_file_or_stdout},
};

mod list;
mod strip;

pub fn rscpio_list(args: ListArgs) -> Result<()> {
    let mut input_ref = BufReader::new(input_file_or_stdin(&args.input)?);
    list::cpio_list(&mut input_ref, &args)
}

pub fn rscpio_strip(args: StripArgs) -> Result<()> {
    let mut input_ref = BufReader::new(input_file_or_stdin(&args.input)?);
    let output_ref = output_file_or_stdout(&args.output)?;
    let output_ref = PartialCursor::new(output_ref);
    let mut output_ref = BufWriter::new(output_ref);
    strip::cpio_strip(&mut input_ref, &mut output_ref, &args)
}
