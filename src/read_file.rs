use std::{fs::File, io::Read};

pub fn read() -> Result<String, std::io::Error>{
  let mut file_path = String::new();
  println!("Please input filepath.");
  std::io::stdin().read_line(&mut file_path)?;
  let file_path = file_path.trim();

  let mut f = File::open(file_path)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  Ok(contents)
}
