#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub id: i32,
    pub author: String,
    pub repo: String,
    pub branch: String,
    pub changeset: String,
    pub message: String,
    pub avatar_url: String,
    pub link: String,
} 