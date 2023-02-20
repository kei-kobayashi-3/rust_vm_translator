pub fn elements(contents: &str) -> Vec<&str> {
  contents
      .lines()
      .filter_map(|item| {
          if item.starts_with('/') {
              None
          } else if let Some(i) = item.find('/') {
              Some(item[..i].trim())
          } else if !item.trim().is_empty() {
              Some(item)
          } else {
              None
          }
      })
      .collect()
}


// pub fn elements<'a>(contents: &'a String) -> Vec<&'a str>{
//   let mut line_elements = Vec::new();
//   'outer: for item in contents.lines(){
//       if item.starts_with("/"){ continue; };
//       for (i, c) in item.chars().enumerate(){
//           if c == '/' {
//               line_elements.push(item[..i].trim());
//               continue 'outer;
//           }
//       }
//       if !item.trim().is_empty(){
//           line_elements.push(&item[..])
//       }
//   }
//   line_elements
// }

#[derive(Debug, PartialEq)]
pub enum CommandType <'a>{
  C_ARITHMEIC(&'a str),
  C_PUSH(&'a str, i32),
  C_POP(&'a str, i32),
  C_LABEL(&'a str),
  C_GOTO(&'a str),
  C_IF(&'a str),
  C_FUNCTION(&'a str, i32),
  C_RETURN,
  C_CALL(&'a str, i32),
}

pub fn get_command_type<'a>(line: &'a str) -> CommandType {
  let mut iter = line.split_whitespace();
  let s = iter.next().unwrap();
  match s {
    "add"|"sub"|"neg"|"eq"|"gt"|"lt"|"and"|"or"|"not" => CommandType::C_ARITHMEIC(s),
    "push"                                            => CommandType::C_PUSH(iter.next().unwrap(), iter.next().unwrap().parse().unwrap()),
    "pop"                                             => CommandType::C_POP(iter.next().unwrap(), iter.next().unwrap().parse().unwrap()),
    "label"                                           => CommandType::C_LABEL(iter.next().unwrap()),
    "goto"                                            => CommandType::C_GOTO(iter.next().unwrap()),
    "if-goto"                                         => CommandType::C_IF(iter.next().unwrap()),
    "function"                                        => CommandType::C_FUNCTION(iter.next().unwrap(), iter.next().unwrap().parse().unwrap()),
    "call"                                            => CommandType::C_CALL(iter.next().unwrap(), iter.next().unwrap().parse().unwrap()),
    "return"                                          => CommandType::C_RETURN,
    &_ => panic!("Problem parsing command."),
  }
}
