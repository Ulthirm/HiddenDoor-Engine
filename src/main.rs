//import the tracing crate
use tracing::{debug, info, warn};
use tracing_subscriber::FmtSubscriber;

// import my config module
mod config;

fn main() {
    // Good MOOOORNING!
    // For debug's sake we're going to print a message to the console
    println!("Engine starting...");

    // Initialize the logging
    let logging_config = config::get_logging_config();

    // Set up the tracing subscriber here
    let subscriber = FmtSubscriber::builder()
        .with_max_level(logging_config)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Logging level: {}", logging_config);


}
