#[derive(Debug)]
pub struct TODO {
    pub line: i32,
    pub text: String,
}
#[derive(Debug)]
pub struct TODOFile {
    pub name: String,
    pub todos: Vec<TODO>,
}
#[derive(Debug)]
pub struct TODODir {
    pub name: String,
    pub files: Vec<TODOFile>,
    pub dirs: Vec<TODODir>,
}
