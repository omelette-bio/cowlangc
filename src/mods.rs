use clap::Subcommand;

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
        println!("Interpreting {} !", file);
      }
    }
  }
}