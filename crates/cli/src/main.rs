use anyhow::Result;

mod cli;
mod commands;
mod termio;

use cli::{Cli, Command, cli};
use commands::{rscpio_list, rscpio_strip};

fn main() -> Result<()> {
    //println!("Hello, world!");
    let Cli {
        command,
        args: mut shared_args,
    } = cli();
    if let Some(directory) = shared_args.directory.take() {
        std::env::set_current_dir(directory)?;
    }
    use Command as E;
    match command {
        E::List(args) => rscpio_list(&args, &shared_args),
        E::Strip(args) => rscpio_strip(&args, &shared_args),
    }
}
