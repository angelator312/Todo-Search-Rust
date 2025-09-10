mod display;
mod log;
mod mylib;
mod structs;
mod utils;

fn main() {
    let todo_dir = mylib::make_todo_from_directory(String::from("./tests"));
    // log::debug("{:?}", todo_dir);
    display::display_directory(&todo_dir, &0);
    // extension::
}
