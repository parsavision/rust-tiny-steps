fn log_message(level: &str, message: &str) {
    println!("{}: {}", level, message);
}
fn log_error(message: &str) {
    log_message("ERROR", message);
}
fn log_info(message: &str) {
    log_message("INFO", message);
}

fn main() {
    log_error("file not found");
    log_info("server started");
}
