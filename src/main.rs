use reqwest;
use scraper::{Html, Selector};
use regex::Regex;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use chrono;
use log::{info, error};

const DISCORD_WEBHOOK_URL: &str = "WEBHOOK_URL";
const COMMITS_URL: &str = "https://commits.facepunch.com/r/rust_reboot";

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_secs()
        .init();

    info!("🚀 Rust commit tracker started - monitoring Facepunch commits");

    let mut last_commit_id: i32 = 0;
    let client = reqwest::Client::new();

    loop {
        match fetch_and_process_commits(&client, &mut last_commit_id).await {
            Ok(_) => {},
            Err(e) => {
                error!("❌ {}", e);
            }
        }

        sleep(Duration::from_secs(50)).await;
    }
}

async fn fetch_and_process_commits(
    client: &reqwest::Client,
    last_commit_id: &mut i32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Request the page
    let response = client.get(COMMITS_URL).send().await?;
    let html_content = response.text().await?;
    let document = Html::parse_document(&html_content);

    // Find the first commit
    let commit_selector = Selector::parse("div.commit.columns")?;
    let commit = document
        .select(&commit_selector)
        .next()
        .ok_or("No commit found")?;

    // Extract the commit id
    let like_id = commit
        .value()
        .attr("like-id")
        .ok_or("No like-id attribute found")?;
    let commit_id: i32 = like_id.parse()?;

    // If this is a new commit
    if commit_id > *last_commit_id {
        // Update the last commit id
        *last_commit_id = commit_id;

        // Extract the commit details
        let author_selector = Selector::parse("div.author")?;
        let author = commit
            .select(&author_selector)
            .next()
            .ok_or("No author found")?
            .text()
            .collect::<String>();

        let repo_selector = Selector::parse("span.repo")?;
        let repo = commit
            .select(&repo_selector)
            .next()
            .ok_or("No repo found")?
            .text()
            .collect::<String>();

        let branch_selector = Selector::parse("span.branch")?;
        let branch = commit
            .select(&branch_selector)
            .next()
            .ok_or("No branch found")?
            .text()
            .collect::<String>();

        let changeset_selector = Selector::parse("span.changeset")?;
        let changeset = commit
            .select(&changeset_selector)
            .next()
            .ok_or("No changeset found")?
            .text()
            .collect::<String>();
        
        // Build changeset link using commit ID
        let changeset_link = format!("https://commits.facepunch.com/{}", commit_id);

        let message_selector = Selector::parse("div.commits-message")?;
        let commit_message = commit
            .select(&message_selector)
            .next()
            .ok_or("No commit message found")?
            .text()
            .collect::<String>();

        info!("🆕 New commit #{} by {} - {}", commit_id, author.trim(), commit_message.trim());

        // Extract avatar URL
        let avatar_selector = Selector::parse("div.avatar")?;
        let avatar_element = commit
            .select(&avatar_selector)
            .next()
            .ok_or("No avatar found")?;
        
        let avatar_html = avatar_element.html();
        let url_regex = Regex::new(r"(https?://[^\s]+)")?;
        let avatar_url = url_regex
            .find(&avatar_html)
            .ok_or("No URL found in avatar")?
            .as_str()
            .trim_end_matches("');")
            .to_string();

        // Send to Discord webhook
        send_discord_webhook(
            client,
            &author,
            &repo,
            &branch,
            &changeset,
            &changeset_link,
            &commit_message,
            &avatar_url,
        ).await?;
        
        info!("✅ Sent to Discord");
    }

    Ok(())
}

async fn send_discord_webhook(
    client: &reqwest::Client,
    author: &str,
    repo: &str,
    branch: &str,
    changeset: &str,
    changeset_link: &str,
    commit_message: &str,
    avatar_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let embed = json!({
        "embeds": [{
            "title": "🔧 New Rust Commit",
            "description": format!("```\n{}\n```", commit_message.trim()),
            "color": 0xCD412B, // Rust orange color
            "author": {
                "name": author.trim(),
                "url": "https://commits.facepunch.com/r/rust_reboot",
                "icon_url": avatar_url
            },
            "fields": [
                {
                    "name": "📁 Repository",
                    "value": format!("`{}`", repo.trim()),
                    "inline": true
                },
                {
                    "name": "🌿 Branch", 
                    "value": format!("`{}`", branch.trim()),
                    "inline": true
                },
                {
                    "name": "🔗 Changeset",
                    "value": format!("[`{}`]({})", changeset.trim(), changeset_link.trim()),
                    "inline": true
                }
            ],
            "footer": {
                "text": "Facepunch Rust Commits",
                "icon_url": "https://i.imgur.com/on47Qk9.png"
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        }]
    });

    let response = client
        .post(DISCORD_WEBHOOK_URL)
        .header("Content-Type", "application/json")
        .json(&embed)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Discord webhook failed with status: {}", response.status()).into());
    }

    Ok(())
}
