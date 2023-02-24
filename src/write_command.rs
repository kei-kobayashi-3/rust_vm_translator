use crate::get_command::CommandType;

pub fn to_string<'a>(command_type: &CommandType, i: &'a mut i32, file_name: &String, function_name: &String) -> String {
  let asm_commands = match command_type {
    CommandType::C_ARITHMEIC("add") => String::from("@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nM=D+M\n@SP\nM=M+1\n"),
    CommandType::C_ARITHMEIC("sub") => String::from("@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nM=M-D\n@SP\nM=M+1\n"),
    CommandType::C_ARITHMEIC("neg") => String::from("@SP\nM=M-1\n@SP\nA=M\nM=-M\n@SP\nM=M+1\n"),
    CommandType::C_ARITHMEIC("eq")  => {
      macro_rules! base_eq {
          () => {"@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nMD=D-M\n@ZERO.{n}\nD;JEQ\n@SP\nA=M\nM=0\n@GO.{n}\n0;JMP\n(ZERO.{n})\n@SP\nA=M\nM=-1\n(GO.{n})\n@SP\nM=M+1\n"}
      }
      let s = format!(base_eq!(), n = i);
      *i += 1;
      s
     },
    CommandType::C_ARITHMEIC("gt")  => {
      macro_rules! base_gt {
        () => {"@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nMD=M-D\n@ZERO.{n}\nD;JGT\n@SP\nA=M\nM=0\n@GO.{n}\n0;JMP\n(ZERO.{n})\n@SP\nA=M\nM=-1\n(GO.{n})\n@SP\nM=M+1\n"}
    }
    let s = format!(base_gt!(), n = i);
    *i += 1;
      s
     },
    CommandType::C_ARITHMEIC("lt")  => {
      macro_rules! base_lt {
        () => {"@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nMD=M-D\n@ZERO.{n}\nD;JLT\n@SP\nA=M\nM=0\n@GO.{n}\n0;JMP\n(ZERO.{n})\n@SP\nA=M\nM=-1\n(GO.{n})\n@SP\nM=M+1\n"}
    }
    let s = format!(base_lt!(), n = i);
    *i += 1;
      s
     },
    CommandType::C_ARITHMEIC("and")         => { String::from("@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nM=D&M\n@SP\nM=M+1\n") },
    CommandType::C_ARITHMEIC("or")          => { String::from("@SP\nM=M-1\n@SP\nA=M\nD=M\n@SP\nAM=M-1\nM=D|M\n@SP\nM=M+1\n") },
    CommandType::C_ARITHMEIC("not")         => { String::from("@SP\nM=M-1\n@SP\nA=M\nM=!M\n@SP\nM=M+1\n") },
    CommandType::C_ARITHMEIC(_) => todo!(),
    CommandType::C_PUSH("constant", n) => { String::from("@") + &n.to_string() + "\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" },
    CommandType::C_PUSH("local", n)    => { String::from("@") + &n.to_string() + "\nD=A\n@LCL\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" },
    CommandType::C_PUSH("argument", n) => { String::from("@") + &n.to_string() + "\nD=A\n@ARG\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" },
    CommandType::C_PUSH("this", n)     => { String::from("@") + &n.to_string() + "\nD=A\n@THIS\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" },
    CommandType::C_PUSH("that", n)     => { String::from("@") + &n.to_string() + "\nD=A\n@THAT\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" },
    CommandType::C_PUSH("pointer", n)  => { String::from("@") + &n.to_string() + "\nD=A\n@THIS\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" },
    CommandType::C_PUSH("temp", n)     => { String::from("@") + &n.to_string() + "\nD=A\n@R5\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" },
    CommandType::C_PUSH("static", n)   => {
      macro_rules! base_static_push {
        () => {"@{fn}.{n}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"}
    }
    let s = format!(base_static_push!(), n = n, fn = file_name);
      s
     },
    CommandType::C_PUSH(_, _) => todo!(),
    CommandType::C_POP("local", n)     => { String::from("@") + &n.to_string() + "\nD=A\n@LCL\nA=M\nD=D+A\n@R15\nM=D\n@SP\nM=M-1\n@SP\nA=M\nD=M\n@R15\nA=M\nM=D\n" },
    CommandType::C_POP("argument", n)  => { String::from("@") + &n.to_string() + "\nD=A\n@ARG\nA=M\nD=D+A\n@R15\nM=D\n@SP\nM=M-1\n@SP\nA=M\nD=M\n@R15\nA=M\nM=D\n" },
    CommandType::C_POP("this", n)      => { String::from("@") + &n.to_string() + "\nD=A\n@THIS\nA=M\nD=D+A\n@R15\nM=D\n@SP\nM=M-1\n@SP\nA=M\nD=M\n@R15\nA=M\nM=D\n" },
    CommandType::C_POP("that", n)      => { String::from("@") + &n.to_string() + "\nD=A\n@THAT\nA=M\nD=D+A\n@R15\nM=D\n@SP\nM=M-1\n@SP\nA=M\nD=M\n@R15\nA=M\nM=D\n" },
    CommandType::C_POP("pointer", n)   => { String::from("@") + &n.to_string() + "\nD=A\n@THIS\nD=D+A\n@R15\nM=D\n@SP\nAM=M-1\nD=M\n@R15\nA=M\nM=D\n" },
    CommandType::C_POP("temp", n)      => { String::from("@") + &n.to_string() + "\nD=A\n@R5\nD=D+A\n@R15\nM=D\n@SP\nAM=M-1\nD=M\n@R15\nA=M\nM=D\n" },
    CommandType::C_POP("static", n)    => {
      macro_rules! base_static_pop {
        () => {"@SP\nAM=M-1\nD=M\n@{fn}.{n}\nM=D\n"}
    }
    let s = format!(base_static_pop!(), n = n, fn = file_name);
      s
     },
    CommandType::C_POP(_, _) => todo!(),
    CommandType::C_LABEL(s)               => String::from("(") + function_name + "$" + s + ")\n",
    CommandType::C_GOTO(s)                => String::from("@") + function_name + "$" + s + "\n0;JMP\n",
    CommandType::C_IF(s)                  => String::from("@SP\nAM=M-1\nD=M\n@") + function_name + "$" + s + "\nD;JNE\n",
    CommandType::C_FUNCTION(s, n)    => {
      macro_rules! base_static_function {
        () => {"({s}) // Function_start\n@{n}\nD=A\n@loopend.{i}\nD;JEQ\n(loop.{i})\n@SP\nA=M\nM=0\n@SP\nM=M+1\n@loop.{i}\nD=D-1;JGT\n(loopend.{i})// Function_end\n"}
    }
    let s = format!(base_static_function!(), s = s, n = n, i = i);
     *i += 1;
      s
     },
    CommandType::C_RETURN                       => String::from("@LCL // Return_start\nD=M\n@R13\nM=D\n@5\nD=A\n@R13\nA=M-D\nD=M\n@R15\nM=D\n@SP\nAM=M-1\nD=M\n@ARG\nA=M\nM=D\n@ARG\nD=M+1\n@SP\nM=D\n@R13\nAM=M-1\nD=M\n@THAT\nM=D\n@R13\nAM=M-1\nD=M\n@THIS\nM=D\n@R13\nAM=M-1\nD=M\n@ARG\nM=D\n@R13\nAM=M-1\nD=M\n@LCL\nM=D\n@R15\nA=M;JMP //Return_end\n"),

    CommandType::C_CALL(s, n)      => {
      macro_rules! base_static_call {
        () => {"@{fn}$return_address.{i} //Call_start\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@{n}\nD=A\n@5\nD=D+A\n@SP\nD=M-D\n@ARG\nM=D\n@SP\nD=M\n@LCL\nM=D\n@{s}\n0;JMP\n({fn}$return_address.{i}) //Call_end\n"}
    }
    let s = format!(base_static_call!(), s = s, n = n, i = i , fn = function_name);
      *i += 1;
      s
     },
  };
  asm_commands
}
