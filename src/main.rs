extern crate clap;

mod file;
mod util;
mod task;

use std::collections::HashMap;
use task::TodoList;

use clap::{Arg, App};


fn main() {
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

  let file_name = "./todo_save.txt";
  let mut file_system = file::FileSystem::new();
  let file = file_system.load(file_name.to_string());
  let data: TodoList = match serde_json::from_str(&file) {
    Ok(x) => x,
    Err(_why) => {
      TodoList {
        todos: HashMap::new(),
      }
    },
  };

  if let Some(mode) = matches.value_of("MODE") {
    match mode {
      // 添加新的todo
      "add" => {
        if let Some(params) = matches.values_of("PARAMETERS") {
          let p_list: Vec<&str> = params.collect();
          let content = p_list[0];
          let newlist = task::add_task(data, content.to_string());
          file_system.write("todo", &serde_json::to_string(&TodoList {
            todos: newlist
          }).unwrap());
          file_system.save(file_name);
        } else {
          println!("请输入参数: -p [Content] \n")
        }
      },
      // 列出所有todo
      "list_all" => {
        task::list_all(&data);
      },
      // 列出某一天的todo
      "list_daily" | "ld" => {
        if let Some(params) = matches.values_of("PARAMETERS") {
          let p_list: Vec<&str> = params.collect();
          let date = p_list[0];
          task::list_daily(&data, date.to_string());
        } else {
          task::list_daily(&data, util::get_cur_ymd());
        }
      },
      // 改变某一项todo的状态, 或者删除一项todo
      "toggle" | "t" | "remove" | "rm" => {
        if let Some(params) = matches.values_of("PARAMETERS") {
          let p_list: Vec<&str> = params.collect();
          let date = p_list[0];
          let index = p_list[1];
          if !date.is_empty() && !index.is_empty() {
            let newlist;
            if mode == "toggle_task" || mode == "t" {
              newlist = task::toggle_task(data, date, index);
            } else {
              newlist = task::remove_task(data, date, index);
            }
            file_system.write("todo", &serde_json::to_string(&TodoList {
              todos: newlist
            }).unwrap());
            file_system.save(file_name);
          } else {
            println!("请输入参数: -p [Date: YYYYMMDD] [Index] \n")
          }
        } else {
          println!("请输入参数: -p [Date: YYYYMMDD] [Index] \n")
        }
      },
      "list_week" | "lw" => {
        let date;
        if let Some(params) = matches.values_of("PARAMETERS") {
          let p_list: Vec<&str> = params.collect();
          date = p_list[0].to_string();
        } else {
          date = util::get_cur_ymd();
        }
        let week_index = util::get_weekday_index(&date);
        for val in data.todos.keys() {
          let c_week_index = util::get_weekday_index(val);
          if c_week_index == week_index {
            task::list_daily(&data, val.to_string());
          }
        }
      },
      _ => println!("没有此命令")
    }
  }
}
