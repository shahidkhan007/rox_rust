#[derive(Clone, Copy)]
pub enum LogLevel {
    Debug,
    Warning,
    Error,
}

#[derive(Clone, Copy)]
pub struct Log {
    pub level: LogLevel,
}

impl Log {
    pub fn error(&self, message: String) {
        println!("\x1b[31m{}\x1b[0m", message);
    }
}
