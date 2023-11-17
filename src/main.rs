use clap::Parser;

mod mods;
mod commands;
mod data;
mod fileparser;
mod map;

use mods::Mods;
// use commands::Commands;
// use data::File;

#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  command: Mods
}

fn main() {
  let args = Cli::parse(); 
  args.command.run();
}
