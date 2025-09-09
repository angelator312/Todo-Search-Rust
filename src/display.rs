// mod structs;
use crate::structs::TODO;
use crate::structs::TODODir;
use crate::structs::TODOFile;

pub fn display_directory(dir: TODODir, depth: usize) {
    let mut dir_name = dir.name;
    dir_name.push_str("/");
    display_content(dir_name, depth.clone());
    let new_depth = depth + 1;
    for dir in dir.dirs {
        display_directory(dir, new_depth);
    }
    for file in dir.files {
        display_file(file, new_depth);
    }
}
pub fn display_file(file: TODOFile, depth: usize) {
    display_content(file.name, depth.clone());
    let new_depth = depth.clone() + 1;
    for todo in file.todos {
        display_todo(todo, new_depth);
    }
}
pub fn display_todo(todo: TODO, depth: usize) {
    let mut content = "❎ ".to_string();
    content.push_str(&todo.text);
    display_content(content, depth);
}
const STRING_ON_NEW_LEVEL: &str = "  ";
fn display_content(content: String, depth: usize) {
    println!("{}{}", STRING_ON_NEW_LEVEL.repeat(depth), content)
}
