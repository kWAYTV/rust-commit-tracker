use rust_commit_tracker::CommitTracker;
use log::error;

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_secs()
        .init();

    let mut tracker = CommitTracker::new();
    
    if let Err(e) = tracker.start().await {
        error!("❌ Fatal error: {}", e);
    }
}
