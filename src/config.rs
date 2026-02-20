use std::env;

/// Bot runtime configuration loaded from environment / .env file.
#[derive(Debug, Clone)]
pub struct Config {
    pub bot_token: String,
    pub mode: BotMode,
    pub polling: PollingConfig,
    pub webhook: WebhookConfig,
    pub admin_id: Option<i64>,
    pub api_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BotMode {
    Polling,
    Webhook,
}

#[derive(Debug, Clone)]
pub struct PollingConfig {
    pub timeout: i64,
    pub limit: i64,
}

#[derive(Debug, Clone)]
pub struct WebhookConfig {
    pub url: String,
    pub port: u16,
    pub path: String,
    pub secret: Option<String>,
    pub max_connections: i64,
    pub drop_pending: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let bot_token = env::var("BOT_TOKEN")
            .map_err(|_| anyhow::anyhow!("BOT_TOKEN must be set in .env or environment"))?;

        let mode = match env::var("BOT_MODE")
            .unwrap_or_else(|_| "polling".to_string())
            .to_lowercase()
            .as_str()
        {
            "webhook" => BotMode::Webhook,
            _ => BotMode::Polling,
        };

        let polling = PollingConfig {
            timeout: env::var("POLLING_TIMEOUT")
                .unwrap_or_else(|_| "30".into())
                .parse()
                .unwrap_or(30),
            limit: env::var("POLLING_LIMIT")
                .unwrap_or_else(|_| "100".into())
                .parse()
                .unwrap_or(100),
        };

        let webhook = WebhookConfig {
            url: env::var("WEBHOOK_URL").unwrap_or_else(|_| "https://example.com".into()),
            port: env::var("WEBHOOK_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .unwrap_or(8080),
            path: env::var("WEBHOOK_PATH").unwrap_or_else(|_| "/webhook".into()),
            secret: env::var("WEBHOOK_SECRET").ok(),
            max_connections: env::var("WEBHOOK_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "40".into())
                .parse()
                .unwrap_or(40),
            drop_pending: env::var("WEBHOOK_DROP_PENDING")
                .unwrap_or_else(|_| "false".into())
                .to_lowercase()
                == "true",
        };

        let admin_id = env::var("ADMIN_ID").ok().and_then(|s| s.parse::<i64>().ok());
        let api_url = env::var("TELEGRAM_API_URL").ok();

        Ok(Config {
            bot_token,
            mode,
            polling,
            webhook,
            admin_id,
            api_url,
        })
    }
}
