use std::fs::File;
use std::io::{SeekFrom, Seek, Write};

pub struct FileWriter {

  out_file: File

}

impl FileWriter {

  pub fn clear(&mut self) {
    if self.out_file.metadata().unwrap().len() as i64 > 0 {
      match self.out_file.set_len(0) {
        Err(_) => println!("Couldn't clear file!"),
        Ok(_) => ()
      };

      match self.out_file.rewind() {
        Err(_) => {
          println!("Couldn't rewind file!");
        },
        Ok(_) => ()
      };
    }
  }

  pub fn write_ln(&mut self, data: &str) {
    self.write(&(data.to_owned() + "\n"));
  }

  pub fn write(&mut self, data: &str) {
    match self.out_file.rewind() {
      Err(_) => {
        println!("Couldn't rewind file!");
      },
      Ok(_) => ()
    };

    let len = self.out_file.metadata().unwrap().len() as i64;
    match self.out_file.set_len(len as u64 + data.as_bytes().len() as u64) {
      Err(err) => {
        println!("Couldn't set file length: {}", err);
      },
      Ok(_) => ()
    };

    match self.out_file.seek(SeekFrom::Current(len))  {
      Err(_) => {
        println!("Couldn't seek in file!");
      },
      Ok(_) => ()
    };

    match self.out_file.write(data.as_bytes()) {
      Err(_) => {
        println!("Couldn't write to file!");
      },
      Ok(_) => ()
    };
  }

}

pub fn open(path: &str) -> Option<FileWriter> {

  match File::open(path) {
    Ok(f) => return Some(FileWriter { out_file: f }),
    Err(_) => match File::create(path) {
      Ok(f) => return Some(FileWriter { out_file: f }),
      Err(_) => None
    }
  }

}