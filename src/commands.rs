// use crate::functions::*;

#[allow(non_camel_case_types)]
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
  pub fn run(&self) {
    match self {
      Commands::moo => {
        println!("moo");
      },
      Commands::mOo => {
        println!("mOo");
      },
      Commands::moO => {
        println!("moO");
      },
      Commands::mOO => {
        println!("mOO");
      },
      Commands::Moo => {
        println!("Moo");
      },
      Commands::MOo => {
        println!("MOo");
      },
      Commands::MoO => {
        println!("MoO");
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
        println!("OOM");
      },
      Commands::oom => {
        println!("oom");
      },
    }
  }
}