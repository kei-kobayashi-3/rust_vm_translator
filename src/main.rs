use std::{fs::File, io::Write};
use vm_translator_rust::{read_file, get_command::{CommandType, ContentWithCommandType, self}, write_command};


fn main() {
    let mut folder_path = String::new();
    println!("input folder path.");
    std::io::stdin().read_line(&mut folder_path).expect("failed reading input words.");
    let folder_path = folder_path.trim();

    let file_content_list = read_file::get_file_content_list(folder_path);

    let content_in_sysinit = file_content_list.iter().find(|&f| f.init );
    let filename_in_sysinit = match content_in_sysinit {
        Some(filecontent) => &filecontent.file_name,
        None => panic!("No sysinit found"),
    };

    let content_with_commands_list = {
        let file_content_list = &file_content_list;
      let mut content_with_commands = Vec::new();
      for content in file_content_list {
          content_with_commands.push( ContentWithCommandType::new(content));
      }
      content_with_commands
    };

    let mut folder_path_list: Vec<&str> = folder_path.split("/").collect();
    let  out_path = folder_path.to_string() + "/" + folder_path_list.pop().unwrap() + ".asm";
    let mut wf = File::create(out_path).expect("problem creating file.");

    let mut i = 0;
    let boot_strap_code = ("@256\nD=A\n@SP\nM=D\n", "call Sys.init 0");
    let boot_string = boot_strap_code.0.to_string() + &write_command::to_string(&get_command::to_command_type(boot_strap_code.1), &mut i, filename_in_sysinit, &String::from("Sys.init"));
    wf.write_all(boot_string.as_bytes()).expect("problem boot strap code.");

    for content_with_commands in content_with_commands_list{
        let file_name = &content_with_commands.file_content.file_name;
        let commands = content_with_commands.command_type;
        let mut function_name = String::new();

        for command in &commands {
            if let CommandType::C_FUNCTION(f_name, _) = command{
                function_name = f_name.to_string();
            }
            wf.write_all(write_command::to_string(command, &mut i, &file_name, &function_name).as_bytes()).expect("problem writing file.");
        }
        // commands.iter().for_each(|command| wf.write_all(write_command::to_string(command, &mut i, &file_name).as_bytes()).expect("problem writing file."));
    }
}
