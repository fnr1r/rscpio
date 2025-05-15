use std::io::{BufReader, BufWriter};

use anyhow::Result;

use crate::{
    cli::{CreateArgs, ListArgs, SharedArgs, StripArgs},
    termio::{PartialCursor, input_file_or_stdin, output_file_or_stdout},
};

mod create;
mod list;
mod strip;

pub fn rscpio_create(args: &CreateArgs, shared_args: &SharedArgs) -> Result<()> {
    let mut input_ref = BufReader::new(input_file_or_stdin(&args.input)?);
    let output_ref = output_file_or_stdout(&args.output)?;
    let output_ref = PartialCursor::new(output_ref);
    let mut output_ref = BufWriter::new(output_ref);
    create::cpio_create(&mut input_ref, &mut output_ref, args, shared_args)
}

pub fn rscpio_list(args: &ListArgs, shared_args: &SharedArgs) -> Result<()> {
    let mut input_ref = BufReader::new(input_file_or_stdin(&args.input)?);
    list::cpio_list(&mut input_ref, args, shared_args)
}

pub fn rscpio_strip(args: &StripArgs, shared_args: &SharedArgs) -> Result<()> {
    let mut input_ref = BufReader::new(input_file_or_stdin(&args.input)?);
    let output_ref = output_file_or_stdout(&args.output)?;
    let output_ref = PartialCursor::new(output_ref);
    let mut output_ref = BufWriter::new(output_ref);
    strip::cpio_strip(&mut input_ref, &mut output_ref, args, shared_args)
}
