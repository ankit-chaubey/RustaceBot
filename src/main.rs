// ════════════════════════════════════════════════════════════════
//  Rustace Bot — @RustaceBot
//  The official showcase bot for the tgbotrs library
//
//  Library:   https://github.com/ankit-chaubey/tgbotrs
//  Bot repo:  https://github.com/ankit-chaubey/RustaceBot
//  Developer: Ankit Chaubey <ankitchaubey.dev@gmail.com>
//  Version:   0.1.0 | tgbotrs 0.1.4 | Telegram Bot API 9.4
// ════════════════════════════════════════════════════════════════

mod config;
mod dispatcher;
mod handlers;

use config::{BotMode, Config};
use dispatcher::dispatch;

use tgbotrs::{Bot, Poller, UpdateHandler};

#[cfg(feature = "webhook")]
use tgbotrs::WebhookServer;

#[cfg(feature = "webhook")]

#[tokio::main]
async fn main() {
    // ── Load .env file ────────────────────────────────────────────────────────
    match dotenv::dotenv() {
        Ok(path) => log::info!("Loaded .env from {:?}", path),
        Err(_) => log::warn!("No .env file found — relying on environment variables"),
    }

    // ── Init logger ───────────────────────────────────────────────────────────
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .format_timestamp_millis()
    .init();

    // ── Load config ───────────────────────────────────────────────────────────
    let cfg = Config::from_env().unwrap_or_else(|e| {
        eprintln!("❌ Configuration error: {}", e);
        eprintln!("💡 Copy .env.example to .env and fill in BOT_TOKEN.");
        std::process::exit(1);
    });

    // ── Print banner ──────────────────────────────────────────────────────────
    print_banner();

    // ── Create bot ────────────────────────────────────────────────────────────
    log::info!("🔐 Connecting to Telegram API...");

    let bot = match &cfg.api_url {
        Some(url) => Bot::with_api_url(&cfg.bot_token, url).await,
        None => Bot::new(&cfg.bot_token).await,
    }
    .unwrap_or_else(|e| {
        eprintln!("❌ Failed to initialize bot: {}", e);
        eprintln!("💡 Check your BOT_TOKEN in .env");
        std::process::exit(1);
    });

    log::info!(
        "✅ Connected as @{} (id={})",
        bot.me.username.as_deref().unwrap_or("unknown"),
        bot.me.id
    );

    // ── Register commands ─────────────────────────────────────────────────────
    log::info!("⚙️  Registering bot commands...");
    match handlers::commands::register_commands(&bot).await {
        Ok(_) => log::info!("✅ Commands registered"),
        Err(e) => log::warn!("⚠️  Could not register commands: {}", e),
    }

    // ── Build update handler ──────────────────────────────────────────────────
    let handler: UpdateHandler = Box::new(|bot, update| {
        Box::pin(async move {
            dispatch(bot, update).await;
        })
    });

    // ── Start polling or webhook ──────────────────────────────────────────────
    match cfg.mode {
        BotMode::Polling => {
            log::info!("🔄 Starting long-polling...");
            log::info!(
                "   timeout={}s  limit={}",
                cfg.polling.timeout,
                cfg.polling.limit
            );

            Poller::new(bot, handler)
                .timeout(cfg.polling.timeout)
                .limit(cfg.polling.limit)
                .start()
                .await
                .unwrap_or_else(|e| {
                    eprintln!("❌ Polling error: {}", e);
                    std::process::exit(1);
                });
        }

        BotMode::Webhook => {
            log::info!("🌐 Starting webhook server...");
            log::info!("   URL: {}{}", cfg.webhook.url, cfg.webhook.path);
            log::info!("   Port: {}", cfg.webhook.port);

            #[cfg(feature = "webhook")]
            {
                let mut server = WebhookServer::new(bot, handler)
                    .port(cfg.webhook.port)
                    .path(cfg.webhook.path.clone())
                    .max_connections(cfg.webhook.max_connections);

                if let Some(ref secret) = cfg.webhook.secret {
                    server = server.secret_token(secret.clone());
                }

                if cfg.webhook.drop_pending {
                    server = server.drop_pending_updates();
                }

                server
                    .start(&cfg.webhook.url)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("❌ Webhook server error: {}", e);
                        std::process::exit(1);
                    });
            }

            #[cfg(not(feature = "webhook"))]
            {
                eprintln!("❌ Webhook mode selected but 'webhook' feature is not enabled.");
                eprintln!("💡 Add features = [\"webhook\"] to tgbotrs in Cargo.toml");
                std::process::exit(1);
            }
        }
    }
}

fn print_banner() {
    println!();
    println!("  ██████╗ ██╗   ██╗███████╗████████╗ █████╗  ██████╗███████╗");
    println!("  ██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔══██╗██╔════╝██╔════╝");
    println!("  ██████╔╝██║   ██║███████╗   ██║   ███████║██║     █████╗  ");
    println!("  ██╔══██╗██║   ██║╚════██║   ██║   ██╔══██║██║     ██╔══╝  ");
    println!("  ██║  ██║╚██████╔╝███████║   ██║   ██║  ██║╚██████╗███████╗");
    println!("  ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝╚══════╝");
    println!();
    println!("  🦀 Rustace Bot v0.1.0 — @RustaceBot");
    println!("  📚 tgbotrs v0.1.4 | Telegram Bot API 9.4");
    println!("  👤 Developer: Ankit Chaubey (github.com/ankit-chaubey)");
    println!("  🔗 https://github.com/ankit-chaubey/RustaceBot");
    println!();
}
