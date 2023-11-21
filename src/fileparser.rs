use std::fs;
use crate::commands::Commands;
use crate::data::File;
use std::rc::Rc;

pub fn parse_file(file: String) -> File {
  let name = file.clone();
  let content = fs::read_to_string(name).expect("Something went wrong reading the file");
  // create vector of strings seperated by spaces and newlines
  let content2: Vec<&str> = content.split_whitespace().collect();
  let mut content3: Vec<Commands> = Vec::new();

  for i in content2 {
    match i {
      "moo" => content3.push(Commands::moo),
      "mOo" => content3.push(Commands::mOo),
      "moO" => content3.push(Commands::moO),
      "mOO" => content3.push(Commands::mOO),
      "Moo" => content3.push(Commands::Moo),
      "MOo" => content3.push(Commands::MOo),
      "MoO" => content3.push(Commands::MoO),
      "MOO" => content3.push(Commands::MOO),
      "OOO" => content3.push(Commands::OOO),
      "MMM" => content3.push(Commands::MMM),
      "OOM" => content3.push(Commands::OOM),
      "oom" => content3.push(Commands::oom),
      _ => (),
    }
  }

  let content3 = Rc::new(content3);

  File {
    content: content3,
    content_iterator: 0,
    data: [0; 30000],
    data_iterator: 0,
  }
}