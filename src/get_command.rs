use crate::read_file::{*, self};

pub fn get_command_type<'a>(file_content_list: &'a Vec<read_file::FileContent>) -> Vec<ContentWithCommandType<'a>> {
  let mut content_with_commands = Vec::new();
  for content in file_content_list {
      content_with_commands.push( ContentWithCommandType::new(content));
  }
  content_with_commands
}

#[derive(Debug)]
pub struct ContentWithCommandType<'a> {
  pub file_content: &'a read_file::FileContent,
  pub command_type: Vec<CommandType<'a>>,
}

impl ContentWithCommandType<'_> {
  pub fn new<'a>(file_content: &'a FileContent) -> ContentWithCommandType {
      let mut commands = Vec::new();
      ContentWithCommandType {
          file_content: file_content,
          command_type: {
              let clean_contents = clean_contents(file_content.contents.as_str());
              for contents in clean_contents{
                  commands.push(to_command_type(contents));
              }
              commands
          },
      }
  }

}



pub fn clean_contents(contents: &str) -> Vec<&str> {
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

pub fn to_command_type<'a>(line: &'a str) -> CommandType {
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
