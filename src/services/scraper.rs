use crate::models::CommitInfo;
use regex::Regex;
use scraper::{Html, Selector};
use std::error::Error;

pub struct CommitScraper {
    client: reqwest::Client,
}

impl CommitScraper {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_latest_commit(&self, url: &str) -> Result<CommitInfo, Box<dyn Error>> {
        let response = self.client.get(url).send().await?;
        let html_content = response.text().await?;
        let document = Html::parse_document(&html_content);

        self.parse_commit_from_html(&document)
    }

    fn parse_commit_from_html(&self, document: &Html) -> Result<CommitInfo, Box<dyn Error>> {
        let commit_selector = Selector::parse("div.commit.columns")?;
        let commit = document
            .select(&commit_selector)
            .next()
            .ok_or("No commit found")?;

        // Extract commit ID
        let like_id = commit
            .value()
            .attr("like-id")
            .ok_or("No like-id attribute found")?;
        let id: i32 = like_id.parse()?;

        // Extract author
        let author_selector = Selector::parse("div.author")?;
        let author = commit
            .select(&author_selector)
            .next()
            .ok_or("No author found")?
            .text()
            .collect::<String>()
            .trim()
            .to_string();

        // Extract repo
        let repo_selector = Selector::parse("span.repo")?;
        let repo = commit
            .select(&repo_selector)
            .next()
            .ok_or("No repo found")?
            .text()
            .collect::<String>()
            .trim()
            .to_string();

        // Extract branch
        let branch_selector = Selector::parse("span.branch")?;
        let branch = commit
            .select(&branch_selector)
            .next()
            .ok_or("No branch found")?
            .text()
            .collect::<String>()
            .trim()
            .to_string();

        // Extract changeset
        let changeset_selector = Selector::parse("span.changeset")?;
        let changeset = commit
            .select(&changeset_selector)
            .next()
            .ok_or("No changeset found")?
            .text()
            .collect::<String>()
            .trim()
            .to_string();

        // Extract commit message
        let message_selector = Selector::parse("div.commits-message")?;
        let message = commit
            .select(&message_selector)
            .next()
            .ok_or("No commit message found")?
            .text()
            .collect::<String>()
            .trim()
            .to_string();

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

        // Build commit link
        let link = format!("https://commits.facepunch.com/{}", id);

        Ok(CommitInfo {
            id,
            author,
            repo,
            branch,
            changeset,
            message,
            avatar_url,
            link,
        })
    }
}

impl Default for CommitScraper {
    fn default() -> Self {
        Self::new()
    }
} 