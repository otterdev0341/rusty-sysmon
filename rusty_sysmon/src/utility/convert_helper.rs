use std::ffi::OsStr;

pub struct ConvertHelper{}

impl ConvertHelper {
    pub fn byte_to_gb(data: u64) -> f64 {
        let result = (data as f64) / (1024.0 * 1024.0 * 1024.0);
        result
    }

    pub fn os_str_to_string(data: &OsStr) -> String {
        let extract_data = match data.to_str() {
            Some(data) => data.to_string(),
            None => "not available".to_string(),
        };
        extract_data
    }
}