use std::{fs::File, io::Write};

use crate::get_elements::CommandType;

pub fn write<'a>(command_type :CommandType, result: &'a mut Vec<String>, i :&'a mut i32 ) -> &'a mut Vec<String> {
  match command_type {
    CommandType::C_ARITHMEIC("add") => { result.push(String::from("@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nM=D+M\n@SP\nM=M+1\n")); result },
    CommandType::C_ARITHMEIC("sub") => { result.push(String::from("@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nM=M-D\n@SP\nM=M+1\n")); result },
    CommandType::C_ARITHMEIC("neg") => { result.push(String::from("@SP\nM=M-1\n@SP\nA=M\nM=-M\n@SP\nM=M+1\n")); result },
    CommandType::C_ARITHMEIC("eq")  => {
      macro_rules! base_eq {
          () => {"@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nMD=D-M\n@ZERO{n}\nD;JEQ\n@SP\nA=M\nM=0\n@GO{n}\n0;JMP\n(ZERO{n})\n@SP\nA=M\nM=-1\n(GO{n})\n@SP\nM=M+1\n"}
      }
      let s = format!(base_eq!(), n = i);
      *i += 1;
       result.push(s); result },
    CommandType::C_ARITHMEIC("gt")  => {
      macro_rules! base_gt {
        () => {"@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nMD=M-D\n@ZERO{n}\nD;JGT\n@SP\nA=M\nM=0\n@GO{n}\n0;JMP\n(ZERO{n})\n@SP\nA=M\nM=-1\n(GO{n})\n@SP\nM=M+1\n"}
    }
    let s = format!(base_gt!(), n = i);
    *i += 1;
     result.push(s); result },
    CommandType::C_ARITHMEIC("lt")  => {
      macro_rules! base_lt {
        () => {"@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nMD=M-D\n@ZERO{n}\nD;JLT\n@SP\nA=M\nM=0\n@GO{n}\n0;JMP\n(ZERO{n})\n@SP\nA=M\nM=-1\n(GO{n})\n@SP\nM=M+1\n"}
    }
    let s = format!(base_lt!(), n = i);
    *i += 1;
     result.push(s); result },
    CommandType::C_ARITHMEIC("and") => { result.push(String::from("@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nM=D&M\n@SP\nM=M+1\n")); result },
    CommandType::C_ARITHMEIC("or")  => { result.push(String::from("@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nM=D|M\n@SP\nM=M+1\n")); result },
    CommandType::C_ARITHMEIC("not") => { result.push(String::from("@SP\nM=M-1\n@SP\nA=M\nM=!M\n@SP\nM=M+1\n")); result },
    CommandType::C_ARITHMEIC(_) => todo!(),
    CommandType::C_PUSH("constant", n) => { result.push(String::from("@") + &n.to_string() + "\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"); result },
    CommandType::C_PUSH(_, _) => todo!(),
    CommandType::C_POP(_, _) => todo!(),
    CommandType::C_LABEL(_) => todo!(),
    CommandType::C_GOTO(_) => todo!(),
    CommandType::C_IF(_) => todo!(),
    CommandType::C_FUNCTION(_, _) => todo!(),
    CommandType::C_RETURN => todo!(),
    CommandType::C_CALL(_, _) => todo!(),
  }
}

pub fn writefile(result: &mut Vec<String>) -> Result<(), std::io::Error>{
  let mut out_path = String::new();
  println!("Please input out_filepath.");
  std::io::stdin().read_line(&mut out_path)?;
  let mut wf = File::create(out_path.trim())?;
  for s in result {
    wf.write_all(s.as_bytes())?;
  }
  Ok(())
}
