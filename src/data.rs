use crate::commands::Commands;

pub struct File {
  pub name: String,
  pub content: Vec<Commands>,
  pub content_iterator: i32,
  pub data: [u8; 30000],
  pub data_iterator: i32,
}