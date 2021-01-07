use crate::util;

use std::collections::HashMap;
use std::path::{Path};
use std::io::{Read, Write};
use std::fs;
use std::fs::File;
use util::calculate_hash;

// 内部使用的数据类型
#[derive(Clone)]
struct InternalFile {
  path: String,
  // Deflate 压缩后的数据
  pub data: String,
  // 压缩前的尺寸
  pub actual_size: usize,
}
impl InternalFile {
}

pub struct FileSystem {
  files: HashMap<String, InternalFile>,
}
impl FileSystem {
  pub fn new() -> FileSystem {
    FileSystem {
      files: HashMap::new(),
    }
  }

  // 写入单个文件
  pub fn write(&mut self, path: &str, data: &str) {
    self.files.insert(
      calculate_hash(&path.to_string().as_bytes()),
      InternalFile {
        path: path.to_string(),
        data: data.to_string(),
        actual_size: data.len(),
      },
    );
  }

  // 将文件系统数据转存到文件
  pub fn save<T: AsRef<Path>>(&self, path: T) {
    let mut f = fs::File::create(&path).unwrap();
    // File contnt
    for (_, file) in &self.files {
      f.write(&file.data.as_bytes()).unwrap();
    }
  }

  pub fn load(&self, path: String) -> String {
    let path_name = Path::new(&path);
    let display = path_name.display();

    let mut file = match File::open(&path_name) {
      Ok(file) => file,
      Err(_why) => {
        self.save(path);
        panic!("文件不存在, 已经新建存储文件, 请重试");
      },
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
      Err(why) => panic!("无法打开文件 {}: {}", display,
      why.to_string()),
      Ok(_) => println!(""),
    }
    s
  }
}


