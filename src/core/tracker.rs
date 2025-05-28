use crate::core::Config;
use crate::services::{CommitScraper, Database, DiscordNotifier};
use log::{debug, error, info};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

pub struct CommitTracker {
    config: Config,
    scraper: CommitScraper,
    notifier: DiscordNotifier,
    database: Database,
}

impl CommitTracker {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config::load_or_create()?;
        let scraper = CommitScraper::new();
        let notifier = DiscordNotifier::new(config.clone());
        let database = Database::new(&config.database.url).await?;

        Ok(Self {
            config,
            scraper,
            notifier,
            database,
        })
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        info!(
            "🚀 {} started - monitoring Facepunch commits",
            self.config.discord.bot_name
        );

        // Get the last sent commit info from database
        if let Some((last_id, changeset)) = self.database.get_last_sent_commit_info().await? {
            info!(
                "📊 Resuming from last sent commit ID: {} ({})",
                last_id, changeset
            );
        } else {
            info!("📊 No previous commits found in database - starting fresh");
        }

        loop {
            if let Err(e) = self.check_for_new_commits().await {
                error!("❌ {}", e);
            }

            sleep(Duration::from_secs(
                self.config.monitoring.check_interval_secs,
            ))
            .await;
        }
    }

    async fn check_for_new_commits(&mut self) -> Result<(), Box<dyn Error>> {
        let result = self
            .scraper
            .fetch_latest_commit(&self.config.monitoring.commits_url)
            .await?;
        let commit = &result.commit;

        // Check if we've already sent this commit
        if self.database.is_commit_sent(commit.id).await? {
            debug!("Commit #{} already sent, skipping", commit.id);
            return Ok(());
        }

        info!(
            "🆕 New commit #{} by {} - {}",
            commit.id,
            commit.author(),
            commit.message
        );

        // Send notification
        self.notifier.send_commit_notification(&result).await?;

        // Mark as sent in database
        self.database
            .mark_commit_sent(
                commit.id,
                &commit.author(),
                &commit.message,
                &commit.branch,
                &commit.changeset,
            )
            .await?;

        info!("✅ Sent to Discord and marked as sent");

        // Periodic cleanup to prevent database from growing too large
        let sent_count = self.database.get_sent_commits_count().await?;
        if sent_count > self.config.database.cleanup_keep_last + 100 {
            self.database
                .cleanup_old_commits(self.config.database.cleanup_keep_last)
                .await?;
        }

        Ok(())
    }
}
