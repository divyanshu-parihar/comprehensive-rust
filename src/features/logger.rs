#[derive(Debug)]
pub enum Level {
    INFO,
}

pub trait Logger {
    fn log(&self, level: Level, message: String);
}

pub struct CustomLogger {}

impl Logger for CustomLogger {
    fn log(&self, level: Level, message: String) {
        dbg!("Level = {:?} - {}", level, message);
    }
}
