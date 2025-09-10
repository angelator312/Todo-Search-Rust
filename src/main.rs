use std::env;

mod display;
mod log;
mod mylib;
mod structs;
mod utils;

const SCAN_DIRECTORY_COMMAND: &str = "scan";
const DONE_FILE_COMMAND: &str = "mark-done";
const DEFAULT_DIRECTORY_FOR_SCAN: &str = "./tests";

/// ### Usage
/// todo-search <command> <path>
///
/// command: scan, mark-done
///
/// path when <command> = scan
///
///     species what directory to scan
///
/// path when <command> = mark-done
///
///`todo-search mark-done <file>:<line>`
///
///file - the file with done task
///line- the line of the task
/// ### Examples:
/// todo-search show ./tests
/// ```
/// TODO.ts
///     ❎ TODO: See everything in app
/// ```
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args = dbg!(args);
    if args.len() > 1 {
        if args[1] == SCAN_DIRECTORY_COMMAND {
            let a = &DEFAULT_DIRECTORY_FOR_SCAN.to_string();
            show_and_scan_directory(if args.len() >= 3 { &args[2] } else { a });
        } else if args[1] == DONE_FILE_COMMAND {
        }
    }
}

fn show_and_scan_directory(path: &String) {
    let todo_dir = mylib::make_todo_from_directory(path);
    // log::debug("{:?}", todo_dir);
    display::display_directory(&todo_dir, &0);
    // extension::
}
