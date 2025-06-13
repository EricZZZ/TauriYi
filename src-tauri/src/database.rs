use sqlx::{sqlite::SqlitePool, Row};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranslationRecord {
    pub id: String,
    pub source_text: String,
    pub translated_text: String,
    pub source_lang: String,
    pub target_lang: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Database {
    pool: Arc<SqlitePool>,
}

impl Database {
    pub async fn new(db_path: PathBuf) -> Result<Self, sqlx::Error> {
        // 确保数据库文件的目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                sqlx::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create database directory: {}", e),
                ))
            })?;
        }

        let database_url = format!("sqlite:{}", db_path.display());
        let pool = Arc::new(SqlitePool::connect(&database_url).await?);
        
        let db = Database { pool };
        db.init_tables().await?;
        Ok(db)
    }

    async fn init_tables(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS translation_history (
                id TEXT PRIMARY KEY,
                source_text TEXT NOT NULL,
                translated_text TEXT NOT NULL,
                source_lang TEXT NOT NULL,
                target_lang TEXT NOT NULL,
                created_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&*self.pool)
        .await?;

        // 创建索引以提高查询性能
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_created_at ON translation_history(created_at);
            CREATE INDEX IF NOT EXISTS idx_source_lang ON translation_history(source_lang);
            CREATE INDEX IF NOT EXISTS idx_target_lang ON translation_history(target_lang);
            "#,
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    pub async fn save_translation(
        &self,
        source_text: &str,
        translated_text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<String, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let created_at = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO translation_history (id, source_text, translated_text, source_lang, target_lang, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(source_text)
        .bind(translated_text)
        .bind(source_lang)
        .bind(target_lang)
        .bind(created_at.to_rfc3339())
        .execute(&*self.pool)
        .await?;

        Ok(id)
    }

    pub async fn get_translation_history(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<TranslationRecord>, sqlx::Error> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let rows = sqlx::query(
            r#"
            SELECT id, source_text, translated_text, source_lang, target_lang, created_at
            FROM translation_history
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*self.pool)
        .await?;

        let mut records = Vec::new();
        for row in rows {
            let created_at_str: String = row.get("created_at");
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
                .with_timezone(&Utc);

            records.push(TranslationRecord {
                id: row.get("id"),
                source_text: row.get("source_text"),
                translated_text: row.get("translated_text"),
                source_lang: row.get("source_lang"),
                target_lang: row.get("target_lang"),
                created_at,
            });
        }

        Ok(records)
    }

    pub async fn search_translations(
        &self,
        query: &str,
        limit: Option<i32>,
    ) -> Result<Vec<TranslationRecord>, sqlx::Error> {
        let limit = limit.unwrap_or(50);
        let search_pattern = format!("%{}%", query);

        let rows = sqlx::query(
            r#"
            SELECT id, source_text, translated_text, source_lang, target_lang, created_at
            FROM translation_history
            WHERE source_text LIKE ? OR translated_text LIKE ?
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(limit)
        .fetch_all(&*self.pool)
        .await?;

        let mut records = Vec::new();
        for row in rows {
            let created_at_str: String = row.get("created_at");
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
                .with_timezone(&Utc);

            records.push(TranslationRecord {
                id: row.get("id"),
                source_text: row.get("source_text"),
                translated_text: row.get("translated_text"),
                source_lang: row.get("source_lang"),
                target_lang: row.get("target_lang"),
                created_at,
            });
        }

        Ok(records)
    }

    pub async fn delete_translation(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM translation_history WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&*self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn clear_history(&self) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM translation_history
            "#,
        )
        .execute(&*self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}