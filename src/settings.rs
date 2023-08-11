use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    device_name: String,
    directshow: bool,
    log_level: String,
    write_log_to_file: bool,
}

impl Settings {
    pub fn new(
        device_name: String,
        directshow: bool,
        log_level: String,
        write_log_to_file: bool,
    ) -> Self {
        Self {
            device_name,
            directshow,
            log_level,
            write_log_to_file,
        }
    }

    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    pub fn directshow(&self) -> bool {
        self.directshow
    }

    pub fn log_level(&self) -> &str {
        &self.log_level
    }

    pub fn write_log_to_file(&self) -> bool {
        self.write_log_to_file
    }
}
