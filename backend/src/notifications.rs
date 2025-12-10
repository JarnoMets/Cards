use crate::models::{Game, GameType};
use crate::db::GameEloChange;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

#[derive(Clone)]
pub struct EmailNotifier {
    smtp_host: String,
    smtp_port: u16,
    smtp_user: String,
    smtp_pass: String,
    from_email: String,
    to_email: String,
    enabled: bool,
    use_tls: bool,
    base_url: String,
}

impl EmailNotifier {
    pub fn from_env() -> Self {
        let enabled = std::env::var("EMAIL_ENABLED")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        // Use TLS by default, but allow disabling for local relays
        let use_tls = std::env::var("SMTP_USE_TLS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(true);

        Self {
            smtp_host: std::env::var("SMTP_HOST").unwrap_or_default(),
            smtp_port: std::env::var("SMTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(587),
            smtp_user: std::env::var("SMTP_USER").unwrap_or_default(),
            smtp_pass: std::env::var("SMTP_PASS").unwrap_or_default(),
            from_email: std::env::var("EMAIL_FROM").unwrap_or_default(),
            to_email: std::env::var("EMAIL_TO").unwrap_or_default(),
            enabled,
            use_tls,
            base_url: std::env::var("BASE_URL").unwrap_or_else(|_| "https://cards.jarnomets.com".to_string()),
        }
    }

    fn build_mailer(&self) -> Result<AsyncSmtpTransport<Tokio1Executor>, Box<dyn std::error::Error + Send + Sync>> {
        if self.use_tls {
            // Use STARTTLS for external SMTP servers
            let creds = Credentials::new(self.smtp_user.clone(), self.smtp_pass.clone());
            Ok(AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_host)?
                .port(self.smtp_port)
                .credentials(creds)
                .build())
        } else {
            // Use plain SMTP for local relay (no auth, no TLS)
            Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.smtp_host)
                .port(self.smtp_port)
                .build())
        }
    }

    fn format_game_type(&self, game: &Game) -> String {
        match &game.game_type {
            GameType::Hearts => "Hearts".to_string(),
            GameType::King => "King".to_string(),
            GameType::DoubleKing => "Double King".to_string(),
            GameType::ColorWhist => "Color Whist".to_string(),
            GameType::Whist => "Whist".to_string(),
            GameType::Manille => "Manille".to_string(),
            GameType::Press => "Press".to_string(),
        }
    }

    fn game_type_url_segment(&self, game: &Game) -> &'static str {
        match &game.game_type {
            GameType::Hearts => "hearts",
            GameType::King => "king",
            GameType::DoubleKing => "double-king",
            GameType::ColorWhist => "color-whist",
            GameType::Whist => "whist",
            GameType::Manille => "manille",
            GameType::Press => "press",
        }
    }

    fn format_game_email(&self, game: &Game, elo_changes: Option<&[GameEloChange]>) -> (String, String) {
        let game_type = self.format_game_type(game);
        let finished_at = chrono::Utc::now();
        
        // Subject: Game type and time
        let subject = format!(
            "{} - Game Finished at {}",
            game_type,
            finished_at.format("%H:%M %d-%m-%Y")
        );
        
        // Build player standings (sorted by score, highest first for most games)
        let mut players_sorted: Vec<_> = game.players.iter().collect();
        
        // Hearts: lowest score wins, others: highest score wins
        if game.game_type == GameType::Hearts {
            players_sorted.sort_by(|a, b| a.total_score.cmp(&b.total_score));
        } else {
            players_sorted.sort_by(|a, b| b.total_score.cmp(&a.total_score));
        }

        let standings = players_sorted
            .iter()
            .enumerate()
            .map(|(i, p)| format!("  {}. {} - {} points", i + 1, p.name, p.total_score))
            .collect::<Vec<_>>()
            .join("\n");

        let winner = players_sorted.first().map(|p| p.name.as_str()).unwrap_or("Unknown");
        
        // Build game URL
        let game_url = format!("{}/en/{}/game/{}", self.base_url, self.game_type_url_segment(game), game.id);
        
        // Format rating changes if available
        let rating_section = if let Some(changes) = elo_changes {
            if !changes.is_empty() {
                let mut sorted_changes = changes.to_vec();
                sorted_changes.sort_by(|a, b| b.elo_change.cmp(&a.elo_change));
                
                let rating_lines = sorted_changes
                    .iter()
                    .map(|c| {
                        let sign = if c.elo_change >= 0 { "+" } else { "" };
                        format!("  {} {} → {} ({}{}) ", c.player_name, c.elo_before, c.elo_after, sign, c.elo_change)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                
                format!("\n\nRating Changes:\n{}", rating_lines)
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        let body = format!(
            "{} Game Results\n\
            {}\n\n\
            Winner: {}\n\n\
            Final Standings:\n{}{}\n\n\
            Rounds played: {}\n\n\
            View game: {}",
            game_type,
            "=".repeat(game_type.len() + 13),
            winner,
            standings,
            rating_section,
            game.current_round,
            game_url
        );

        (subject, body)
    }

    pub async fn notify_game_finished(&self, game: &Game, elo_changes: Option<&[GameEloChange]>) {
        if !self.enabled {
            return;
        }

        if let Err(e) = self.send_game_finished_email(game, elo_changes).await {
            log::error!("Failed to send game finished email: {}", e);
        }
    }

    async fn send_game_finished_email(&self, game: &Game, elo_changes: Option<&[GameEloChange]>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (subject, body) = self.format_game_email(game, elo_changes);

        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(self.to_email.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        let mailer = self.build_mailer()?;

        mailer.send(email).await?;
        log::info!("Game finished email sent for game {}", game.id);

        Ok(())
    }

    pub async fn send_test_email(&self, latest_game: Option<&Game>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.enabled {
            return Err("Email notifications are not enabled".into());
        }

        if self.smtp_host.is_empty() || self.from_email.is_empty() || self.to_email.is_empty() {
            return Err("Email configuration is incomplete".into());
        }

        let (subject, body) = if let Some(game) = latest_game {
            let (game_subject, game_body) = self.format_game_email(game, None);
            
            let subject = format!("[TEST] {}", game_subject);
            
            let body = format!(
                "This is a TEST email from the Card Game Scorer application.\n\
                Below is a preview of how game notification emails will look.\n\n\
                {}\n\n\
                ---\n\
                Email Configuration:\n\
                - SMTP Host: {}\n\
                - SMTP Port: {}\n\
                - From: {}\n\
                - To: {}\n\
                - TLS: {}\n\n\
                Sent at: {}",
                "-".repeat(60),
                self.smtp_host,
                self.smtp_port,
                self.from_email,
                self.to_email,
                if self.use_tls { "Enabled" } else { "Disabled" },
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            );
            
            // Combine: separator, game body, then config
            let full_body = format!(
                "{}\n\n{}\n\n{}",
                body.lines().take(3).collect::<Vec<_>>().join("\n"),
                game_body,
                body.lines().skip(3).collect::<Vec<_>>().join("\n")
            );
            
            (subject, full_body)
        } else {
            let subject = "Card Game Scorer - Test Email".to_string();
            let body = format!(
                "This is a test email from the Card Game Scorer application.\n\n\
                No games have been played yet, so there is no game data to preview.\n\n\
                If you received this email, your email notifications are configured correctly!\n\n\
                Configuration:\n\
                - SMTP Host: {}\n\
                - SMTP Port: {}\n\
                - From: {}\n\
                - To: {}\n\
                - TLS: {}\n\n\
                Sent at: {}",
                self.smtp_host,
                self.smtp_port,
                self.from_email,
                self.to_email,
                if self.use_tls { "Enabled" } else { "Disabled" },
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            );
            (subject, body)
        };

        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(self.to_email.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        let mailer = self.build_mailer()?;

        mailer.send(email).await?;
        log::info!("Test email sent successfully");

        Ok(())
    }

    /// Send game finished email to a specific player
    pub async fn send_player_game_email(
        &self, 
        game: &Game, 
        player_email: &str,
        elo_changes: Option<&[GameEloChange]>
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.enabled {
            return Err("Email notifications are not enabled".into());
        }

        let (subject, body) = self.format_game_email(game, elo_changes);

        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(player_email.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        let mailer = self.build_mailer()?;
        mailer.send(email).await?;
        
        log::info!("Player game email sent to {} for game {}", player_email, game.id);
        Ok(())
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
