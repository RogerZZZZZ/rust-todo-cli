## 0. 前言

**Lingling insurance**

自己刚开始学习Rust不久, 写这个简单的cli也是为了练手, 代码不优雅也请各路大神指点, 我定虚心求教! 起初接触到rust是因为WebAssembly, 有不少人用rust编译为WSAM, 作为一个合格的前端娱乐圈工作者, 还是希望对此有一定的了解. 

该CLI实现一个持久化的TODO list. 那废话不多说, 咱们开始吧!

## 1. 开始

### 1.1 rust安装

如果你是用的是Macos或者是linux都可以简单的通过下面的命令安装rust

```bash
curl <https://sh.rustup.rs> -sSf | sh
```

如果你使用的window系统, 可以上[rustup](https://rustup.rs/)下载对应exe文件.

### 1.2 cargo的使用

`cargo`是`rust`包内自带的包管理工具, 类似npm, 同时也提供了一些cli来编译或者初始化一个项目

> cargo init

```bash
// 初始化一个项目
cargo init xxx-project 
```

执行结束以后会在执行目录下生产下面的目录结构

```bash
|
  .git/
  |
  .gitignore
  |
  Cargo.toml
  |
  src/
```

其中需要解释的是`Cargo.toml`文件, 其作用和`npm`中`package.json`相同, 对项目信息和包依赖进行一些说明

```toml
[package]
name = "rust-cli"
version = "0.1.0"
authors = ["rogerjluo <rogerjluo@tencent.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
serde = { version = "1.0", features = ["derive"] }
clap = "~2.32"
serde_json = "1.0"
chrono = "0.4"
```

> cargo build

改命令是将项目进行预编译, 安装你在toml文件中的依赖至`target`文件夹内, 并生成或者更新`Cargo.lock`文件对依赖的版本进行管理(作用类似package-lock.json).

```bash
cargo build
   Compiling proc-macro2 v1.0.24
   Compiling autocfg v1.0.1
    ...
    ...
   Compiling chrono v0.4.19
   Compiling rust-cli v0.1.0 (/Users/rogerjluo/Documents/project/rust-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 28.25s
```

> cargo run build

编译后执行二进制文件, 比如在`src/main.rs`文件中写入

```rust
fn main () {
  println!("Hello World!")
}
```

```bash
cargo run build
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rust-cli build`
Hello World!
```

## 2. 文件存储和读取

因为我们要做一个持久化数据的todolist, 所以文件操作是必不可少的, 这里就用到了标准库中的`std::io::{Read, Write}`以及`std::fs`.

首先我们需要声明一些`struct`

```rust
struct InternalFile {
  path: String,
  pub data: String,
  pub actual_size: usize,
}

pub struct FileSystem {
  files: HashMap<String, InternalFile>,
}
```

> 写文件

这里使用HashMap的原因是不希望多次写文件, 所以将多次写入操作合并之后再写入.

```rust
// 将写入数据先写入到内存中
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
  for (_, file) in &self.files {
    f.write(&file.data.as_bytes()).unwrap();
  }
}
```

> 读文件

```rust
pub fn load(&self, path: String) -> String {
  let path_name = Path::new(&path);
  let display = path_name.display();

  let mut file = match File::open(&path_name) {
    Ok(file) => file,
    Err(_why) => {
      // 如果文件不存在则创建一个文件
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
  s // 返回文件内容
}
```

## 3. JSON的序列化和反序列化

当然为了方便对文件内容进行操作, 我们对写入和读取进行了序列化和反序列的过程, 使用到的是`serde_json`, 实现类似JS中的`JSON.parse`以及`JSON.stringify`

> 序列化

```rust
serde_json::to_string(TodoList)
```

> 反序列化

```rust
let data: TodoList = match serde_json::from_str(&file) {
  Ok(x) => x,
  Err(_why) => {
    // 如果文件为空或者出错, 则创建一个空的list返回
    TodoList {
      todos: HashMap::new(),
    }
  },
};
```

这里我们是可以指定反序列化的目标结构, 如果结构不匹配则无法完成反序列化, 但是前提是需要在`struct`上加上一些声明

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoTask {
  content: String,
  is_done: bool,
  c_time: String,
  u_time: String,
  index: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
  pub todos: HashMap<String, Vec<TodoTask>>
}
```

## 4. clap

基础的功能有了, 接下来我们需要获取命令行中传入的参数并执行不同的逻辑, `cargo`中 `cargo run -- [参数]`可以将参数传入到程序中, 但我们这里将使用一个叫做`clap`的库来简化这个过程.

```rust
let help_instru = r#"命令名:
    add(a)          -p [Content]                  : 添加当天的todo
    list_all(la)                                  : 打印全部todo
    list_daily(ld)  -p [Date: YYYY-MM-DD]         : 打印某一天的todo, 不带参数默认打印当天
    remove(rm)      -p [Date: YYYY-MM-DD] [Index] : 删除某一项todo
    toggle(t)       -p [Date: YYYY-MM-DD] [Index] : 改变某一项todo的状态, 完成/待完成
    list_week(lw)   -p [Date: YYYY-MM-DD]         : 打印某一周的todo, 方便生成周报
  "#;
  let matches = App::new("Todo")
    .version("0.0.1")
    .author("rogerjluo")
    .about("rust实现的todolist")
    .arg(Arg::with_name("MODE")
      .short("m")
      .long("mode")
      .takes_value(true)
      .help(help_instru))
    .arg(Arg::with_name("PARAMETERS")
      .short("p")
      .long("params")
      .multiple(true)
      .takes_value(true)
      .help("参数"))
    .get_matches();
```

进行了声明之后, 我们如何在程序中获取到传入的参数呢

```rust
// 如果是参数只支持单个value, 此处match的名字与之前声明的Arg::with_name("MODE")对应
if let Some(mode) = matches.value_of("MODE") {
  match mode {
    "add" => {

    },
    "list_all" => {

    },
    // ....
    _ => println!("没有此命令")
  }
}

// 支持multiple value
if let Some(params) = matches.values_of("PARAMETERS") {
  let p_list: Vec<&str> = params.collect();
  let date = p_list[0]; // 参数一
  let index = p_list[1]; // 参数二
} else {
  println!("请输入参数: -p [Date: YYYYMMDD] [Index] \n")
}
```

## 5. 举个栗子 🌰

这里我们只列举其中的两个具体实现~ 如果感兴趣可以去仓库中查看别的代码, 代码地址在文末.

> 写入一项todo

这里我们只写入当天的todo, 所以传入的参数只需要写入的内容即可, 命令为: `cargo run -- -m add -p HelloWorld`

```rust
"add" => {
  if let Some(params) = matches.values_of("PARAMETERS") {
    let p_list: Vec<&str> = params.collect();
    let content = p_list[0];
    // add_task的作用就是将新的todo插入到对应日期的todo列表中, 这里就不列举代码了
    let newlist = task::add_task(data, content.to_string());
    file_system.write("todo", &serde_json::to_string(&TodoList {
      todos: newlist
    }).unwrap());
    file_system.save(file_name);
  } else {
    println!("请输入参数: -p [Content] \n")
  }
},
```

执行后, 我们会看到指定写入文件内会有下面的信息

```json
{"todos":{"2020-12-12":[{"content":"HelloWorld","is_done":false,"c_time":"2020-12-12 07:21:31","u_time":"2020-12-12 07:21:31","index":"0"}]}}
```

> 列举某一天所在周的全部todo

我们要实现这个功能, 需要计算出某一日期所在的周在该年中的index, 这里借助了`chrono`库提供的api实现

```rust
use chrono::prelude::*;
use chrono::{NaiveDate};
pub fn get_weekday_index(date: &str) -> u32 {
  let utc = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
  // utc.ordinal()可以获取日期在该年中的index
  let day_index = utc.ordinal();
  (day_index / 7) + 1
}
```

```rust
"list_week" | "lw" => {
  let date;
  // 这里判断是否传入日期参数, 没有传入默认为当天
  if let Some(params) = matches.values_of("PARAMETERS") {
    let p_list: Vec<&str> = params.collect();
    date = p_list[0].to_string();
  } else {
    date = util::get_cur_ymd();
  }
  let week_index = util::get_weekday_index(&date);
  for val in data.todos.keys() {
    let c_week_index = util::get_weekday_index(val);
    // 判断是否在同一周, 如果是则打印
    if c_week_index == week_index {
      task::list_daily(&data, val.to_string());
    }
  }
},
```

执行命令为: `cargo run -- -m lw`

```bash
cargo run -- -m lw               
   Compiling rust-cli v0.1.0 (/Users/rogerjluo/Documents/project/rust-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 2.00s
     Running `target/debug/rust-cli -m lw`

2020-12-12:

0. [Need Handle] HelloWorld
```

## 6. 结尾

文章只是介绍了项目中的部分代码以及功能, 完整版已经实现的功能包括:
- 写入新的todo
- 更改todo状态, 待处理和已完成间进行切换
- 删除某个todo
- 列出所有todo
- 列出本周的todo list
- 列出某一天的todo list

(这里没有列出所有的代码, 如果你感兴趣, 请移步到: [git.oa](https://github.com/RogerZZZZZ/rust-todo-cli)) 如果觉得还不错, 帮忙点个star吧, 初学者举步维艰

如果大家对代码有什么建议的话希望能告诉我, 这将继续鞭策我前进, 感谢~