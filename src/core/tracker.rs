use crate::core::Config;
use crate::services::{CommitScraper, DiscordNotifier};
use log::{info, error};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

pub struct CommitTracker {
    config: Config,
    scraper: CommitScraper,
    notifier: DiscordNotifier,
    last_commit_id: i32,
}

impl CommitTracker {
    pub fn new() -> Self {
        let config = Config::new();
        let scraper = CommitScraper::new();
        let notifier = DiscordNotifier::new(config.clone());

        Self {
            config,
            scraper,
            notifier,
            last_commit_id: 0,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        info!("🚀 Rust commit tracker started - monitoring Facepunch commits");

        loop {
            if let Err(e) = self.check_for_new_commits().await {
                error!("❌ {}", e);
            }

            sleep(Duration::from_secs(self.config.check_interval_secs)).await;
        }
    }

    async fn check_for_new_commits(&mut self) -> Result<(), Box<dyn Error>> {
        let commit = self.scraper.fetch_latest_commit(&self.config.commits_url).await?;

        if commit.id > self.last_commit_id {
            self.last_commit_id = commit.id;
            
            info!("🆕 New commit #{} by {} - {}", commit.id, commit.author, commit.message);
            
            self.notifier.send_commit_notification(&commit).await?;
            
            info!("✅ Sent to Discord");
        }

        Ok(())
    }
}

impl Default for CommitTracker {
    fn default() -> Self {
        Self::new()
    }
} 