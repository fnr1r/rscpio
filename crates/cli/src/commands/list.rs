use std::io::Read;

use anyhow::Result;
use cpiolib::ext::CpioIterExt;

use crate::cli::ListArgs;

pub fn cpio_list(reader: &mut impl Read, args: &ListArgs) -> Result<()> {
    let eol = if args.zero { '\0' } else { '\n' };
    for entry in reader.read_as_cpio() {
        let header = entry?.header;
        let txt = header.name.to_string_lossy();
        print!("{}{}", txt, eol);
    }
    Ok(())
}
