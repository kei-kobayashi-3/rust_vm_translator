use std::{fs::File, io::Read};

pub fn read() -> Result<(String, String), std::io::Error>{
  let mut file_path = String::new();
  println!("Please input filepath.");
  std::io::stdin().read_line(&mut file_path)?;
  let file_path = file_path.trim();
  let file_name = &file_path[(file_path.rfind("/").unwrap()+ 1)..file_path.find(".").unwrap()];
  let mut f = File::open(file_path)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  Ok((contents, file_name.to_string()))
}
