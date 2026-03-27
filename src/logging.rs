use std::io::Write;
use std::sync::Mutex;
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

struct Logger {
    log_file: File,
    log_path: String,
    logs: HashMap<SystemTime, String>,
}

impl Logger {
    fn new() -> Self {
        let log_path = String::from("/var/log/container_health.log");
        let file = File::create(&log_path).expect("Failed to create log file");
        Logger {
            log_file: file,
            log_path: log_path.clone(),
            logs: HashMap::new(),
        }
    }

    fn log(&mut self, message: &str) {
        let time = SystemTime::now();
        let log_entry = format!("[{}] {}
", time, message);
        
        self.logs.insert(time, log_entry.clone());
        
        self.log_file.write_all(log_entry.as_bytes()).expect("Failed to write log entry");
    }
}

pub fn init_logger() {
    let mut logger = LOGGER.lock().unwrap();
    logger.log("Initializing logger...");
}

pub fn log_message(message: &str) {
    let mut logger = LOGGER.lock().unwrap();
    logger.log(message);
}

pub fn get_logs() -> HashMap<SystemTime, String> {
    let logger = LOGGER.lock().unwrap();
    logger.logs.clone()
}