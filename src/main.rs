use vm_translator_rust::{read_file, get_elements, write_command};

// chat-gpt refactoring
fn main() {
    match read_file::read() {
        Ok((contents, file_name)) => {
            let line_elements = get_elements::elements(&contents);
            let enum_elements = line_elements.iter().map(|s| get_elements::get_command_type(s)).collect::<Vec<_>>();
            let mut result = Vec::new();
            let mut i = 0;
            for e in enum_elements {
                write_command::write(e, &mut result, &mut i, &file_name);
            }
            if let Err(e) = write_command::writefile(&mut result) {
                panic!("Problem writing the file: {}", e);
            }
        },
        Err(e) => panic!("Problem opening the file: {}", e),
    }
}

// fn main() {
//     let (contents, file_name) = match read_file::read(){
//         Ok(s) => s,
//         Err(e) => panic!("Problem openinng the file: {}", e),
//     };
//     let line_elements = get_elements::elements(&contents);
//     let mut enum_elements = Vec::new();
//     for s in line_elements{
//         let e_enum = get_elements::get_cmmand_type(s);
//         enum_elements.push(e_enum);
//     }

//     let mut result = Vec::new();
//     let mut i = 0;

//     for e in enum_elements {
//         write_command::write(e, &mut result, &mut i, &file_name);
//     }

//     if let Err(e) = write_command::writefile(&mut result){
//         panic!("Problem writing the file: {}", e);
//     }
// }
