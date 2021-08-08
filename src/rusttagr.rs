extern crate anyhow;

use std;
use crate::pos_tagging;
use crate::pos_tagging::POSModel;

fn try_tag(input: &str) -> anyhow::Result<std::vec::Vec<std::vec::Vec<pos_tagging::POSTag>>> {
  let format_vec = [input]; 
  //    Set-up model
  let pos_model = POSModel::new(Default::default())?;
  //    Run model
  Ok(pos_model.predict(&format_vec))
} 

#[no_mangle]
pub fn rust_tag_r(input: &str) -> String {
  let output = match try_tag(input) {
    Ok(x) => x,
    Err(x) => panic!("{}", x)
  };

  let mut str_out : String = "".to_owned();
  for pos_tag in output {
    str_out.push_str(&format!("{:?}", pos_tag));
  }
  str_out
}
