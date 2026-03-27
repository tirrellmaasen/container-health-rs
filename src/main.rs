use std::sync::Once;
static INIT_LOGGER: Once = Once::new();

fn main() {
    INIT_LOGGER.call_once(|| {
        init_logger();
    });

    log_message("Container health check started.");

    // Simulate a health check process
    log_message("Checking container status...");
    log_message("All containers are healthy.");

    let logs = get_logs();
    for (time, message) in logs {
        println!("{} - {}", time, message);
    }
}