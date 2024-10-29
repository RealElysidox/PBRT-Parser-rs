#[derive(Debug, Clone)]
pub struct FileLoc {
    filename: String,
    line: u32,
    column: u32,
}

impl FileLoc {
    pub fn new(filename: String) -> Self {
        FileLoc {
            filename: filename.to_string(),
            line: 1,
            column: 0,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}:{}", self.filename, self.line, self.column)
    }
}