pub struct ConvertHelper{}

impl ConvertHelper {
    pub fn mb_to_gb(data: u64) -> f64 {
        let result = (data as f64) / (1024.0 * 1024.0 * 1024.0);
        result
    }
}