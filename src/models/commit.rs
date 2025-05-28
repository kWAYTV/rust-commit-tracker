use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CommitsResponse {
    pub total: u32,
    pub skip: u32,
    pub take: u32,
    pub results: Vec<CommitInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitInfo {
    pub id: i32,
    pub repo: String,
    pub branch: String,
    pub changeset: String,
    pub created: String,
    pub likes: u32,
    pub dislikes: u32,
    pub message: String,
    pub user: CommitUser,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitUser {
    pub name: String,
    pub avatar: String,
}

impl CommitInfo {
    pub fn link(&self) -> String {
        format!("https://commits.facepunch.com/{}", self.id)
    }

    pub fn avatar_url(&self) -> &str {
        &self.user.avatar
    }

    pub fn author(&self) -> &str {
        &self.user.name
    }
}
