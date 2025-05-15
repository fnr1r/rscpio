use anyhow::Result;

mod cli;
mod impls;
mod termio;

use cli::{Cli, Command, cli};
use impls::{rscpio_list, rscpio_strip};

fn main() -> Result<()> {
    //println!("Hello, world!");
    let Cli { command, directory } = cli();
    if let Some(directory) = directory {
        std::env::set_current_dir(directory)?;
    }
    use Command as E;
    match command {
        E::List(args) => rscpio_list(args),
        E::Strip(args) => rscpio_strip(args),
    }
}
