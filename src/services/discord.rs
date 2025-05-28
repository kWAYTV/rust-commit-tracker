use crate::core::Config;
use crate::models::{DiscordEmbed, EmbedData, EmbedAuthor, EmbedField, EmbedFooter};
use crate::services::scraper::CommitResult;
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

    pub async fn send_commit_notification(&self, result: &CommitResult) -> Result<(), Box<dyn Error>> {
        let embed = self.build_embed(result);
        
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

    fn build_embed(&self, result: &CommitResult) -> DiscordEmbed {
        let commit = &result.commit;
        
        DiscordEmbed {
            embeds: vec![EmbedData {
                title: "🔧 New Rust Commit".to_string(),
                description: format!("```\n{}\n```", commit.message),
                color: self.config.rust_color(),
                author: EmbedAuthor {
                    name: commit.author().to_string(),
                    url: self.config.monitoring.commits_url.clone(),
                    icon_url: commit.avatar_url().to_string(),
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
                        value: format!("[`{}`]({})", commit.changeset, commit.link()),
                        inline: true,
                    },
                ],
                footer: EmbedFooter {
                    text: format!("{} • Commit {} of {}", 
                        self.config.discord.bot_name,
                        self.format_number(result.total_commits - result.position + 1),
                        self.format_number(result.total_commits)
                    ),
                    icon_url: self.config.discord.bot_avatar_url.clone(),
                },
                timestamp: chrono::Utc::now().to_rfc3339(),
            }],
        }
    }

    fn format_number(&self, num: u32) -> String {
        // Add commas to large numbers for readability
        let num_str = num.to_string();
        let chars: Vec<char> = num_str.chars().collect();
        let mut result = String::new();
        
        for (i, &ch) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(ch);
        }
        
        result
    }
} 