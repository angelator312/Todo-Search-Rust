struct TODO {
    line: i32,
    text: String,
}

struct TODOFile {
    name: String,
    todos: Vec<TODO>,
}

struct TODODir {
    name: String,
    files: Vec<TODOFile>,
    dirs: Vec<TODODir>,
}

fn main() {
    // let list_files = list_files_in_directory(String::from("./tests"));
    // print_everything_in_array(list_files);
    // let list_files = list_files_in_directory(String::from("./tests/app"));
    // print_everything_in_array(list_files);
    // let list_files = list_files_in_directory(String::from("./tests/lib"));
    // print_everything_in_array(list_files);
    make_todo_from_file(String::from("./tests/app/TODO.ts"));
}

use std::fs;

fn list_files_in_directory(path: String) -> Vec<String> {
    let mut arr: Vec<String> = vec![];
    // for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
    for file in fs::read_dir(path).unwrap() {
        let file_name = file.unwrap().path().display().to_string();
        arr.push(file_name);
    }
    return arr;
}

fn print_everything_in_array(arr: Vec<String>) {
    for e in arr.into_iter() {
        println!("path:{}", e)
    }
}
// use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn make_todo_from_file(path: String) {
    let mut file = File::open(path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");
    let lines = content.lines();
    for e in lines {
        make_todo_from_line(String::from(e));
    }
}

fn make_todo_from_line(line: String) -> TODO {
    if let Some(todo) = line.find("TODO:") {
        return TODO {
            text: { String::from(((line[todo..]).chars()).as_str()) },
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
