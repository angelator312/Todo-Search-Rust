use crate::log;
use crate::structs::TODO;
use crate::structs::TODODir;
use crate::structs::TODOFile;
use crate::utils;

use std::fs;
/// path:PathOfDirectory;only_dirs or only_files
pub fn list_files_or_directories_in_directory(path: &String, files_or_dirs: bool) -> Vec<String> {
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

pub fn list_files_in_directory(path: &String) -> Vec<String> {
    return list_files_or_directories_in_directory(path, false);
}

pub fn list_dirs_in_directory(path: &String) -> Vec<String> {
    return list_files_or_directories_in_directory(path, true);
}

// pub fn print_everything_in_array(arr: Vec<String>) {
//     for e in arr.into_iter() {
//         log::log_path(&e)
//     }
// }
// use regex::Regex;
use std::fs::File;
use std::io::Read;

pub fn make_todo_from_directory(path: &String) -> TODODir {
    let mut todo_dir = TODODir {
        name: utils::from_path_to_name(path),
        files: vec![],
        dirs: vec![],
    };
    for e in list_dirs_in_directory(path).into_iter() {
        log::log_dir_name(&e);
        todo_dir.dirs.push(make_todo_from_directory(&e));
    }
    for e in list_files_in_directory(path).into_iter() {
        log::log_file_name(&e);
        todo_dir.files.push(make_todo_from_file(&e));
    }
    return todo_dir;
}

pub fn make_todo_from_file(path: &String) -> TODOFile {
    let name = utils::from_path_to_name(path);
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
        now_todo = make_todo_from_line(&e.to_string());
        if now_todo.line == -1 {
            continue;
        }
        now_todo.line = line;
        file_todo.todos.push(now_todo);
        line += 1;
    }
    return file_todo;
}

pub fn make_todo_from_line(line: &String) -> TODO {
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
}
