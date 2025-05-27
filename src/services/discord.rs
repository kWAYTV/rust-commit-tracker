use crate::core::Config;
use crate::models::{CommitInfo, DiscordEmbed, EmbedData, EmbedAuthor, EmbedField, EmbedFooter};
use chrono;
use std::error::Error;

pub struct DiscordNotifier {
    client: reqwest::Client,
    config: Config,
}

impl DiscordNotifier {
    pub fn new(config: Config) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    pub async fn send_commit_notification(&self, commit: &CommitInfo) -> Result<(), Box<dyn Error>> {
        let embed = self.build_embed(commit);
        
        let response = self.client
            .post(&self.config.discord.webhook_url)
            .header("Content-Type", "application/json")
            .json(&embed)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Discord webhook failed with status: {}", response.status()).into());
        }

        Ok(())
    }

    fn build_embed(&self, commit: &CommitInfo) -> DiscordEmbed {
        DiscordEmbed {
            embeds: vec![EmbedData {
                title: "🔧 New Rust Commit".to_string(),
                description: format!("```\n{}\n```", commit.message),
                color: self.config.rust_color(),
                author: EmbedAuthor {
                    name: commit.author.clone(),
                    url: self.config.monitoring.commits_url.clone(),
                    icon_url: commit.avatar_url.clone(),
                },
                fields: vec![
                    EmbedField {
                        name: "📁 Repository".to_string(),
                        value: format!("`{}`", commit.repo),
                        inline: true,
                    },
                    EmbedField {
                        name: "🌿 Branch".to_string(),
                        value: format!("`{}`", commit.branch),
                        inline: true,
                    },
                    EmbedField {
                        name: "🔗 Changeset".to_string(),
                        value: format!("[`{}`]({})", commit.changeset, commit.link),
                        inline: true,
                    },
                ],
                footer: EmbedFooter {
                    text: self.config.discord.bot_name.clone(),
                    icon_url: self.config.discord.bot_avatar_url.clone(),
                },
                timestamp: chrono::Utc::now().to_rfc3339(),
            }],
        }
    }
} 