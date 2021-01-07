use crate::util;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub fn add_task(todolist: TodoList, content: String) -> HashMap<String, Vec<TodoTask>> {
  let cur_date = util::get_cur_ymd();
  let mut mutate_todos = todolist.todos;
  let todos = mutate_todos.entry(cur_date.to_string()).or_insert(Vec::new());
  todos.push(TodoTask {
    content: content,
    is_done: false,
    c_time: util::get_cur_time(),
    u_time: util::get_cur_time(),
    index: todos.len().to_string(),
  });
  mutate_todos
}

pub fn toggle_task(todolist: TodoList, date: &str, index: &str) -> HashMap<String, Vec<TodoTask>> {
  let mut mutate_todos = todolist.todos;
  let todos = mutate_todos.entry(date.to_string()).or_insert(Vec::new());
  let target = todos.iter().position(|x| {
    x.index == index
  });
  match target {
    Some(i) => {
      todos.push(TodoTask {
        content: (*todos[i].content).to_string(),
        is_done: !todos[i].is_done,
        c_time: (*todos[i].c_time).to_string(),
        u_time: util::get_cur_time(),
        index: (*todos[i].index).to_string(),
      });
      todos.remove(i);
    },
    None => println!("修改项不存在"),
  };
  mutate_todos
}

pub fn remove_task(todolist: TodoList, date: &str, index: &str) -> HashMap<String, Vec<TodoTask>> {
  let mut mutate_todos = todolist.todos;
  let todos = mutate_todos.entry(date.to_string()).or_insert(Vec::new());
  let target = todos.iter().position(|x| {
    x.index == index
  });
  match target {
    Some(i) => {
      todos.remove(i);
    },
    None => println!(),
  };
  mutate_todos
}

fn print_todos(list: &Vec<TodoTask>, date: String) {
  if let Some(todo) = Some(list) {
    if todo.len() > 0 {
      println!("{}:\n", date);
      todo.iter().enumerate().for_each(|(index, x)| {
        let status = match x.is_done {
          true => "[Done]",
          false => "[Need Handle]",
        };
        println!("{}. {} {}\n", index, status, x.content);
      })
    } else {
      println!("没有待办事项哦");
    }
  }
}

pub fn list_daily(todolist: &TodoList, date: String) {
  match todolist.todos.get(&date) {
    Some(daily) => print_todos(daily, date),
    None => println!("{}没有待办事项", date),
  }
}

pub fn list_all(todolist: &TodoList) {
  for val in todolist.todos.keys() {
    match todolist.todos.get(val) {
      Some(x) => print_todos(x, val.to_string()),
      None => println!("没有待办事项"),
    }
  }
}