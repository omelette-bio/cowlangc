use crate::data::File;

#[allow(non_snake_case)]
pub fn mOo(file : &mut File) {
  if file.data_iterator == 0 {
    panic!("Error: data iterator cant be negative");
  }
  file.data_iterator -= 1;
  file.content_iterator += 1;
}

#[allow(non_snake_case)]
pub fn moO(file : &mut File) {
  if file.data_iterator == 29999 {
    panic!("Error: data iterator cant be more than 29999");
  }
  file.data_iterator += 1;
  file.content_iterator += 1;
}

#[allow(non_snake_case)]
pub fn MOo(file : &mut File) {
  file.data[file.data_iterator as usize] -= 1;
  file.content_iterator += 1;
}

#[allow(non_snake_case)]
pub fn MoO(file : &mut File) {
  file.data[file.data_iterator as usize] += 1;
  file.content_iterator += 1;
}

#[allow(non_snake_case)]
pub fn Moo(file : &mut File) {
  print!("{}", file.data[file.data_iterator as usize] as char);
  file.content_iterator += 1;
}

#[allow(non_snake_case)]
pub fn OOM(file : &mut File) {
  print!("{}", file.data[file.data_iterator as usize]);
  file.content_iterator += 1;
}

#[allow(non_snake_case)]
pub fn OOO(file : &mut File) {
  file.data[file.data_iterator as usize] = 0;
  file.content_iterator += 1;
}