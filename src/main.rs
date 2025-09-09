mod display;
mod structs;
mod utils;
use structs::TODO;
use structs::TODODir;
use structs::TODOFile;

fn main() {
    // let list_files = list_files_in_directory(String::from("./tests"));
    // print_everything_in_array(list_files);
    // let list_files = list_files_in_directory(String::from("./tests/app"));
    // print_everything_in_array(list_files);
    // let list_files = list_files_in_directory(String::from("./tests/lib"));
    // print_everything_in_array(list_files);
    // println!(
    //     "TODOS:{:?}",
    //     (make_todo_from_file(String::from("./tests/app/TODO.ts"))).todos
    // );
    let todo_dir = make_todo_from_directory(String::from("./tests"));
    println!("{:?}", todo_dir);
    display::display_directory(todo_dir, 0);
}

use std::fs;
/// path:PathOfDirectory;only_dirs or only_files
fn list_files_or_directories_in_directory(path: String, files_or_dirs: bool) -> Vec<String> {
    let mut arr: Vec<String> = vec![];
    // for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
    for file in fs::read_dir(path).unwrap() {
        let file2 = file.unwrap();
        // If "file" is directory
        if file2.metadata().unwrap().is_file() == files_or_dirs {
            continue;
        }
        let file_name = file2.path().display().to_string();
        arr.push(file_name);
    }
    return arr;
}

fn list_files_in_directory(path: String) -> Vec<String> {
    return list_files_or_directories_in_directory(path, false);
}

fn list_dirs_in_directory(path: String) -> Vec<String> {
    return list_files_or_directories_in_directory(path, true);
}

fn print_everything_in_array(arr: Vec<String>) {
    for e in arr.into_iter() {
        println!("path:{}", e)
    }
}
// use regex::Regex;
use std::fs::File;
use std::io::Read;

fn make_todo_from_directory(path: String) -> TODODir {
    let mut todo_dir = TODODir {
        name: utils::from_path_to_name(path.clone()),
        files: vec![],
        dirs: vec![],
    };
    for e in list_dirs_in_directory(path.clone()).into_iter() {
        println!("Dir:{}", e);
        todo_dir.dirs.push(make_todo_from_directory(e));
    }
    for e in list_files_in_directory(path).into_iter() {
        println!("file:{}", e);
        todo_dir.files.push(make_todo_from_file(e));
    }
    return todo_dir;
}

fn make_todo_from_file(path: String) -> TODOFile {
    let name = utils::from_path_to_name(path.clone());
    let mut file = File::open(path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");
    let lines = content.lines();
    let mut now_todo: TODO;
    let mut file_todo = TODOFile {
        name,
        todos: vec![],
    };
    let mut line = 0;
    for e in lines {
        now_todo = make_todo_from_line(String::from(e));
        if now_todo.line == -1 {
            continue;
        }
        now_todo.line = line;
        file_todo.todos.push(now_todo);
        line += 1;
    }
    return file_todo;
}

fn make_todo_from_line(line: String) -> TODO {
    if let Some(todo) = line.find("TODO:") {
        return TODO {
            text: { utils::string_sub_to_end(line, todo) },
            line: 0,
        };
    } else {
        return TODO {
            text: String::new(),
            line: -1,
        };
    }
    // println!("is_todo:{} at line: {}", !is_todo.is_none(), line);
    // if is_todo.is_none() {
    // }
}
