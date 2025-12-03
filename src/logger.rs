use chrono::Local;

pub fn log_request(method: &str, path: &str, status: u16) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("[{}] {} {} - {}", timestamp, method, path, status);
}
