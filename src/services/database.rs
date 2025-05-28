use log::{debug, info};
use sqlx::{Row, SqlitePool};
use std::error::Error;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        // Create the database file if it doesn't exist (for SQLite)
        if database_url.starts_with("sqlite:") {
            let file_path = database_url.strip_prefix("sqlite:").unwrap_or(database_url);
            if !std::path::Path::new(file_path).exists() {
                debug!("Creating SQLite database file: {}", file_path);
                std::fs::File::create(file_path)?;
            }
        }

        let pool = SqlitePool::connect(database_url).await?;

        let db = Self { pool };
        db.initialize().await?;

        Ok(db)
    }

    async fn initialize(&self) -> Result<(), Box<dyn Error>> {
        // Create the sent_commits table if it doesn't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sent_commits (
                id INTEGER PRIMARY KEY,
                commit_id INTEGER NOT NULL UNIQUE,
                author TEXT NOT NULL,
                message TEXT NOT NULL,
                branch TEXT NOT NULL,
                changeset TEXT NOT NULL,
                sent_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        debug!("Database initialized successfully");
        Ok(())
    }

    pub async fn is_commit_sent(&self, commit_id: i32) -> Result<bool, Box<dyn Error>> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM sent_commits WHERE commit_id = ?")
            .bind(commit_id)
            .fetch_one(&self.pool)
            .await?;

        let count: i64 = row.get("count");
        Ok(count > 0)
    }

    pub async fn mark_commit_sent(
        &self,
        commit_id: i32,
        author: &str,
        message: &str,
        branch: &str,
        changeset: &str,
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
            INSERT INTO sent_commits (commit_id, author, message, branch, changeset)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(commit_id)
        .bind(author)
        .bind(message)
        .bind(branch)
        .bind(changeset)
        .execute(&self.pool)
        .await?;

        debug!("Marked commit {} as sent", commit_id);
        Ok(())
    }

    pub async fn get_last_sent_commit_id(&self) -> Result<Option<i32>, Box<dyn Error>> {
        let row = sqlx::query("SELECT MAX(commit_id) as max_id FROM sent_commits")
            .fetch_one(&self.pool)
            .await?;

        let max_id: Option<i32> = row.get("max_id");
        Ok(max_id)
    }

    pub async fn get_last_sent_commit_info(&self) -> Result<Option<(i32, String)>, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT commit_id, changeset FROM sent_commits ORDER BY commit_id DESC LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let commit_id: i32 = row.get("commit_id");
            let changeset: String = row.get("changeset");
            Ok(Some((commit_id, changeset)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_sent_commits_count(&self) -> Result<i64, Box<dyn Error>> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM sent_commits")
            .fetch_one(&self.pool)
            .await?;

        let count: i64 = row.get("count");
        Ok(count)
    }

    pub async fn cleanup_old_commits(&self, keep_last: i64) -> Result<(), Box<dyn Error>> {
        // Keep only the last N commits to prevent database from growing too large
        sqlx::query(
            r#"
            DELETE FROM sent_commits 
            WHERE id NOT IN (
                SELECT id FROM sent_commits 
                ORDER BY commit_id DESC 
                LIMIT ?
            )
            "#,
        )
        .bind(keep_last)
        .execute(&self.pool)
        .await?;

        info!("Cleaned up old commits, keeping last {}", keep_last);
        Ok(())
    }
}
