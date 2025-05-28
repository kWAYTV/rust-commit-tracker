use crate::models::{CommitInfo, CommitsResponse};
use std::error::Error;

pub struct CommitScraper {
    client: reqwest::Client,
}

#[derive(Debug)]
pub struct CommitResult {
    pub commit: CommitInfo,
    pub total_commits: u32,
    pub position: u32, // Position in the list (1 = latest)
}

impl CommitScraper {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_latest_commit(&self, url: &str) -> Result<CommitResult, Box<dyn Error>> {
        let response = self.client.get(url).send().await?;
        let commits_response: CommitsResponse = response.json().await?;

        let commit = commits_response
            .results
            .into_iter()
            .next()
            .ok_or("No commits found in response")?;

        Ok(CommitResult {
            commit,
            total_commits: commits_response.total,
            position: 1, // Latest commit is always position 1
        })
    }
}

impl Default for CommitScraper {
    fn default() -> Self {
        Self::new()
    }
}
