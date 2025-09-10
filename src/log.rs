const DEBUG_LOGS_ALLOWED_ON_RELEASE: bool = false;
const DEBUG_LOGS_ALLOWED_ON_DEV: bool = true;
const DEBUG_LOGS_ALLOWED: bool = if cfg!(debug_assertions) {
    DEBUG_LOGS_ALLOWED_ON_DEV
} else {
    DEBUG_LOGS_ALLOWED_ON_RELEASE
};

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

// pub fn log_path(path: &String) {
//     if DEBUG_LOGS_ALLOWED {
//         println!("path:{}", *path);
//     }
// }
