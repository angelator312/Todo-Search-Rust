static DEBUG_LOGS_ALLOWED: bool = true;

pub fn log_file_name(name: &String) {
    if DEBUG_LOGS_ALLOWED {
        println!("file:{}", *name);
    }
}

pub fn log_dir_name(name: &String) {
    if DEBUG_LOGS_ALLOWED {
        println!("dir:{}", *name);
    }
}

pub fn log_path(path: &String) {
    if DEBUG_LOGS_ALLOWED {
        println!("path:{}", *path);
    }
}
