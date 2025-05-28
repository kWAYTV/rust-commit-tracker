use log::error;
use rust_commit_tracker::CommitTracker;

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_secs()
        .init();

    let mut tracker = match CommitTracker::new().await {
        Ok(tracker) => tracker,
        Err(e) => {
            error!("❌ Failed to initialize tracker: {}", e);
            return;
        }
    };

    if let Err(e) = tracker.start().await {
        error!("❌ Fatal error: {}", e);
    }
}
