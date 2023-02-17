pub mod read_file;
pub mod get_elements;
pub mod write_command;

#[cfg(test)]
mod tests {
    use crate::{get_elements};

 #[test]
 fn enum_works(){
  let s1 = "push constant 7";
  let s2 = "goto that";
  let s3 = "call getCom 6";
  assert_eq!(get_elements::get_cmmand_type(s1), get_elements::CommandType::C_PUSH("constant", 7));
  assert_eq!(get_elements::get_cmmand_type(s2), get_elements::CommandType::C_GOTO("that"));
  assert_eq!(get_elements::get_cmmand_type(s3), get_elements::CommandType::C_CALL("getCom", 6));
 }
}
