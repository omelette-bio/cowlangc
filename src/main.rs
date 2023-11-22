use clap::Parser;

mod mods;
mod commands;
mod data;
mod fileparser;
mod map;
mod functions;

use mods::Mods;

#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  command: Mods
}

fn main() {
  let args = Cli::parse(); 
  args.command.run();
}
