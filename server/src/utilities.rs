use std::time::{SystemTime, UNIX_EPOCH};

pub fn timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or("0000000".to_string())
}
