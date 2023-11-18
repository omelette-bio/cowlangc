use clap::Subcommand;
use crate::fileparser::parse_fichier;

#[derive(Subcommand)]
pub enum Mods {
  /// Compile the cowlang file
  Compile{
    /// The cowlang file to compile
    file: String,
  },
  /// Interpret the cowlang file
  Interpret{
    /// The cowlang file to interpret
    file: String,
  },
}

impl Mods {
  pub fn run(&self) {
    match self {
      Mods::Compile {file} => {
        println!("Compiling {} !", file);
      },
      Mods::Interpret {file} => {
        let mut file = parse_fichier(file.to_string());
        let cont = file.content.clone();
        for i in 0..cont.len() {
          let command = &cont[i];
          command.run(&mut file);
        }
      }
    }
  }
}