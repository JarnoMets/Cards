//! Player settings for email preferences

use super::core::{Database, DbResult};
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSettings {
    pub player_name: String,
    pub email: Option<String>,
    pub game_notifications: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDailyStats {
    pub date: String,
    pub emails_sent: i32,
}

impl Database {
    /// Get player settings, returns default if not found
    pub async fn get_player_settings(&self, player_name: &str) -> DbResult<PlayerSettings> {
        let normalized = player_name.trim().to_lowercase();
        
        let row = sqlx::query(
            "SELECT player_name, email, game_notifications 
             FROM player_settings 
             WHERE LOWER(TRIM(player_name)) = $1"
        )
        .bind(&normalized)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(PlayerSettings {
                player_name: row.get("player_name"),
                email: row.get("email"),
                game_notifications: row.get("game_notifications"),
            }),
            None => Ok(PlayerSettings {
                player_name: player_name.trim().to_string(),
                email: None,
                game_notifications: false,
            }),
        }
    }

    /// Update player settings (upsert)
    pub async fn update_player_settings(&self, settings: &PlayerSettings) -> DbResult<()> {
        let trimmed_name = settings.player_name.trim();
        
        sqlx::query(
            "INSERT INTO player_settings (player_name, email, game_notifications, updated_at)
             VALUES ($1, $2, $3, NOW())
             ON CONFLICT (player_name) 
             DO UPDATE SET 
                email = EXCLUDED.email,
                game_notifications = EXCLUDED.game_notifications,
                updated_at = NOW()"
        )
        .bind(trimmed_name)
        .bind(&settings.email)
        .bind(settings.game_notifications)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get all players with game notifications enabled who played in a game
    pub async fn get_notification_recipients(&self, player_names: &[String]) -> DbResult<Vec<PlayerSettings>> {
        if player_names.is_empty() {
            return Ok(vec![]);
        }

        // Build the query with normalized names
        let normalized: Vec<String> = player_names.iter()
            .map(|n| n.trim().to_lowercase())
            .collect();

        let rows = sqlx::query(
            "SELECT player_name, email, game_notifications 
             FROM player_settings 
             WHERE LOWER(TRIM(player_name)) = ANY($1) 
               AND game_notifications = TRUE 
               AND email IS NOT NULL 
               AND email != ''"
        )
        .bind(&normalized)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|row| PlayerSettings {
            player_name: row.get("player_name"),
            email: row.get("email"),
            game_notifications: row.get("game_notifications"),
        }).collect())
    }

    /// Increment daily email counter and check if limit reached
    /// Returns (current_count, can_send)
    pub async fn check_and_increment_email_count(&self, daily_limit: i32) -> DbResult<(i32, bool)> {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

        // Use upsert to atomically get and increment
        let row = sqlx::query(
            "INSERT INTO email_daily_stats (date, emails_sent, last_email_at)
             VALUES ($1, 1, NOW())
             ON CONFLICT (date) 
             DO UPDATE SET 
                emails_sent = email_daily_stats.emails_sent + 1,
                last_email_at = NOW()
             RETURNING emails_sent"
        )
        .bind(&today)
        .fetch_one(&self.pool)
        .await?;

        let count: i32 = row.get("emails_sent");
        let can_send = count <= daily_limit;

        Ok((count, can_send))
    }

    /// Get today's email stats
    pub async fn get_email_daily_stats(&self) -> DbResult<EmailDailyStats> {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

        let row = sqlx::query(
            "SELECT date::text, emails_sent FROM email_daily_stats WHERE date = $1::date"
        )
        .bind(&today)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(EmailDailyStats {
                date: row.get("date"),
                emails_sent: row.get("emails_sent"),
            }),
            None => Ok(EmailDailyStats {
                date: today,
                emails_sent: 0,
            }),
        }
    }

    /// Rename player in settings table
    pub async fn rename_player_settings(&self, old_name: &str, new_name: &str) -> DbResult<u64> {
        let old_normalized = old_name.trim().to_lowercase();
        
        // Check if settings exist for old name
        let existing = sqlx::query(
            "SELECT email, game_notifications FROM player_settings WHERE LOWER(player_name) = $1"
        )
        .bind(&old_normalized)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_none() {
            return Ok(0);
        }

        // Delete old and insert/update new
        let result = sqlx::query(
            "UPDATE player_settings SET player_name = $1, updated_at = NOW() WHERE LOWER(player_name) = $2"
        )
        .bind(new_name.trim())
        .bind(&old_normalized)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
