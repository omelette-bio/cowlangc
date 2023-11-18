use crate::functions;
use crate::data::File;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum Commands {
  moo,
  mOo,
  moO,
  mOO,
  Moo,
  MOo,
  MoO,
  MOO,
  OOO,
  MMM,
  OOM,
  oom,
}

impl Commands {
  pub fn run(&self, file: &mut File) {
    match self {
      Commands::moo => {
        println!("moo");
      },
      Commands::mOo => {
        functions::mOo(file);
      },
      Commands::moO => {
        functions::moO(file);
      },
      Commands::mOO => {
        println!("mOO");
      },
      Commands::Moo => {
        println!("Moo");
      },
      Commands::MOo => {
        functions::MOo(file);
      },
      Commands::MoO => {
        functions::MoO(file);
      },
      Commands::MOO => {
        println!("MOO");
      },
      Commands::OOO => {
        println!("OOO");
      },
      Commands::MMM => {
        println!("MMM");
      },
      Commands::OOM => {
        functions::OOM(file);
      },
      Commands::oom => {
        println!("oom");
      },
    }
  }
}