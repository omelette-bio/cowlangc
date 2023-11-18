use crate::commands::Commands;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct File {
  pub content: Rc<Vec<Commands>>,
  pub content_iterator: i32,
  pub data: [u8; 30000],
  pub data_iterator: i32,
}