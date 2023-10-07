#[derive(Debug)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError {
        LoxError { line, message }
    }

    pub fn report(err: LoxError, loc: usize) {
        println!("[Line {}] Error: '{}': {}", err.line, err.message, loc)
    }
}
