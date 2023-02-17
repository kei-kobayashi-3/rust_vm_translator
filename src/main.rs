use vm_translator_rust::{read_file, get_elements, write_command};

fn main() {
    let contents = match read_file::read(){
        Ok(s) => s,
        Err(e) => panic!("Problem openinng the file: {}", e),
    };
    let line_elements = get_elements::elements(&contents);

    let mut enum_elements = Vec::new();
    for s in line_elements{
        let e_enum = get_elements::get_cmmand_type(s);
        enum_elements.push(e_enum);
    }

    let mut result = Vec::new();
    for e in enum_elements {
        write_command::write(e, &mut result);
    }

    if let Err(e) = write_command::writefile(&mut result){
        panic!("Problem writing the file: {}", e);
    }
}
