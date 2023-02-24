use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct FileContent {
    pub file_name: String,
    pub file_path: String,
    pub contents: String,
    pub init: bool,
}

impl FileContent {
    pub fn new(file_name: &str, file_path: &str) -> FileContent {
        let config = Self::read_contents(file_path);
        FileContent {
            file_name: file_name.to_string(),
            file_path: file_path.to_string(),
            contents:  config.0,
            init: config.1,
         }
    }
    fn read_contents(file_path: &str) -> (String, bool) {
        let mut contents = String::new();
        let mut f = File::open(file_path).expect("Problem opening file.");
        f.read_to_string(&mut contents).expect("Problem reading file.");
        let init_is_contain = contents.contains("Sys.init 0");
        (contents, init_is_contain)
    }
}

pub fn get_file_content_list(folder_path: &str) -> Vec<FileContent>{
    let pattern = format!("{}/*.vm", folder_path);
    let file_paths = glob::glob(&pattern);
    let mut config: Vec<FileContent> = Vec::new();
    for entry in file_paths.unwrap(){
        match entry {
            Ok(path) => {
                if let Some(file_path) = path.to_str(){
                    let file_name = &file_path[(file_path.rfind("/").unwrap()+ 1)..file_path.find(".").unwrap()];
                    config.push(FileContent::new(file_name, file_path));
                }
            },
            Err(e) => println!("{}", e),
        }
    }
    config
}
