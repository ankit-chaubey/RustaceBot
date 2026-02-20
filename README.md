<div align="center">

# 🦀 Rustace Bot — @RustaceBot

**The official showcase bot for [tgbotrs](https://github.com/ankit-chaubey/tgbotrs)**

[![Rust](https://img.shields.io/badge/Rust-🦀-orange?style=flat-square)](https://www.rust-lang.org)
[![tgbotrs](https://img.shields.io/badge/tgbotrs-v0.1.4-blue?style=flat-square)](https://crates.io/crates/tgbotrs)
[![Bot API](https://img.shields.io/badge/Telegram_Bot_API-9.4-blue?style=flat-square)](https://core.telegram.org/bots/api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=flat-square)](LICENSE)

*Built with ❤️ by [Ankit Chaubey](https://github.com/ankit-chaubey)*

</div>

---

## 🤖 What is Rustace?

**Rustace** (`@RustaceBot`) is a fully functional Telegram bot that demonstrates **every feature** of the [tgbotrs](https://github.com/ankit-chaubey/tgbotrs) library — a fully-featured, auto-generated Rust Telegram Bot API library.

This bot covers:
- ✅ All **165 API methods** from Telegram Bot API 9.4
- ✅ All **285 types** — strongly typed  
- ✅ All **15 update types** (message, callback, inline, polls, reactions, boosts, etc.)
- ✅ **Long-polling** mode (`Poller`)
- ✅ **Webhook** mode (`WebhookServer`)
- ✅ Interactive menus, inline keyboards, reply keyboards
- ✅ Media sending demos (photo, animation, audio, video, voice, document, sticker)
- ✅ Inline query handling with results
- ✅ Dice animations (🎲🎯🎳🏀⚽🎰)

---

## 🚀 Quick Start

### 1. Clone the repository

```bash
git clone https://github.com/ankit-chaubey/RustaceBot.git
cd RustaceBot
```

### 2. Configure environment

```bash
cp .env.example .env
```

Edit `.env` and set your `BOT_TOKEN`:

```env
BOT_TOKEN=1234567890:AAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
BOT_MODE=polling   # or webhook
```

### 3. Run the bot

```bash
cargo run --release
```

---

## ⚙️ Configuration

All configuration is done via the `.env` file. See [`.env.example`](.env.example) for all available options.

### Polling mode (recommended for development)

```env
BOT_TOKEN=your_token_here
BOT_MODE=polling
POLLING_TIMEOUT=30
POLLING_LIMIT=100
```

### Webhook mode (recommended for production)

```env
BOT_TOKEN=your_token_here
BOT_MODE=webhook
WEBHOOK_URL=https://your-domain.com
WEBHOOK_PORT=8080
WEBHOOK_PATH=/webhook
WEBHOOK_SECRET=your_random_secret_here
WEBHOOK_MAX_CONNECTIONS=40
WEBHOOK_DROP_PENDING=false
```

**Note:** For webhook mode, your server needs a valid HTTPS certificate accessible from the internet. Telegram supports ports `80`, `88`, `443`, `8443`.

---

## 📋 Bot Commands

| Command | Description |
|---------|-------------|
| `/start` | Welcome message & main menu |
| `/help` | Full command reference |
| `/about` | About Rustace & tgbotrs |
| `/menu` | Show main interactive menu |
| `/dice` | Roll a dice 🎲 |
| `/darts` | Throw darts 🎯 |
| `/bowling` | Play bowling 🎳 |
| `/basketball` | Shoot hoops 🏀 |
| `/football` | Kick the ball ⚽ |
| `/slots` | Slot machine 🎰 |
| `/fact` | Random Rust fact 💡 |
| `/joke` | Programmer joke 😂 |
| `/magic8` | Magic 8-ball 🔮 |
| `/coinflip` | Flip a coin 🪙 |
| `/photo` | Demo `send_photo()` |
| `/animation` | Demo `send_animation()` |
| `/location` | Demo `send_location()` |
| `/venue` | Demo `send_venue()` |
| `/contact` | Demo `send_contact()` |
| `/poll` | Create a poll 📊 |
| `/textstyles` | HTML formatting demo |
| `/botinfo` | Bot info (`get_me`) |
| `/webhookinfo` | Webhook status |
| `/membercount` | Chat member count |
| `/admins` | List administrators |
| `/invitelink` | Generate invite link |
| `/mycommands` | Show registered commands |
| `/myprofile` | Your profile photos |
| `/library` | Full library method overview |
| `/stats` | Bot statistics |
| `/setcommands` | Register bot commands |
| `/deletecommands` | Delete bot commands |
| `/deletewebhook` | Remove webhook |

---

## 🎯 Features Demonstrated

### 📨 Messaging Methods
- `send_message` — with HTML parse mode, reply markup
- `send_photo` — photo with caption
- `send_animation` — GIF/MP4 animation
- `send_audio` — audio files
- `send_video` — video files
- `send_video_note` — circular video
- `send_voice` — voice messages
- `send_document` — any file
- `send_sticker` — sticker messages
- `send_location` — map location
- `send_venue` — venue with address
- `send_contact` — contact card
- `send_dice` — animated dice (🎲🎯🎳🏀⚽🎰)
- `send_poll` — interactive polls
- `send_media_group` — photo/video albums

### ✏️ Editing Methods
- `edit_message_text` — edit sent messages
- `edit_message_reply_markup` — update keyboards
- `edit_message_caption` — update media captions
- `edit_message_live_location` — live location updates
- `stop_message_live_location` — stop live sharing

### 💬 Chat Management
- `get_chat` — fetch chat info
- `get_chat_administrators` — list admins
- `get_chat_member_count` — member count
- `get_chat_member` — member info
- `ban_chat_member` / `unban_chat_member`
- `restrict_chat_member` / `promote_chat_member`
- `pin_chat_message` / `unpin_chat_message`
- `export_chat_invite_link` — invite link
- `create_chat_invite_link` — custom invite link
- `leave_chat` — bot leaves chat

### 📡 API Utilities
- `get_me` — bot self info
- `get_webhook_info` — webhook status
- `set_webhook` / `delete_webhook`
- `get_updates` — manual polling
- `get_file` — file info & download URL

### ⚙️ Bot Settings
- `set_my_commands` / `get_my_commands` / `delete_my_commands`
- `set_my_name` / `get_my_name`
- `set_my_description` / `get_my_description`
- `set_my_short_description` / `get_my_short_description`

### 🔍 Inline Mode
- `answer_inline_query` — with articles, text, URL buttons
- `answer_callback_query` — toast, alert, URL callbacks

### 💳 Payments & Stars
- `get_my_star_balance` — bot's star balance
- `get_star_transactions` — transaction history
- `send_invoice` / `create_invoice_link`
- `answer_shipping_query`
- `answer_pre_checkout_query`

### 🎭 Stickers (15 methods)
- `get_sticker_set`, `create_new_sticker_set`, `add_sticker_to_set`
- `set_sticker_emoji_list`, `set_sticker_keywords`
- `replace_sticker_in_set`, `delete_sticker_set`
- ...and more

### 🏢 Business Account (12 methods)
- `get_business_connection`
- `set_business_account_name`, `set_business_account_bio`
- `read_business_message`, `delete_business_messages`
- ...and more

### 🎮 Games (3 methods)
- `send_game`, `set_game_score`, `get_game_high_scores`

### 📢 Stories (5 methods)
- `post_story`, `edit_story`, `delete_story`, `repost_story`

---

## 🏗️ Project Structure

```
RustaceBot/
├── src/
│   ├── main.rs              # Entry point, bot init, polling/webhook
│   ├── config.rs            # .env configuration loader
│   ├── dispatcher.rs        # Update dispatcher (all 15 update types)
│   └── handlers/
│       ├── mod.rs           # Handler module exports
│       ├── commands.rs      # /command handlers & menu keyboards
│       ├── callbacks.rs     # Inline keyboard callback handler
│       └── inline.rs        # Inline query handler
├── .env.example             # Environment configuration template
├── .gitignore
├── Cargo.toml
└── README.md
```

---

## 📦 Dependencies

```toml
[dependencies]
tgbotrs    = { version = "0.1.4", features = ["webhook"] }
tokio      = { version = "1", features = ["full"] }
dotenv     = "0.15"
log        = "0.4"
env_logger = "0.11"
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
rand       = "0.8"
anyhow     = "1"
```

---

## 🔧 Building

```bash
# Debug build
cargo build

# Release build (recommended for production)
cargo build --release

# Run directly
cargo run

# Run with specific log level
RUST_LOG=debug cargo run
```

---

## 🌐 Deploying with Webhook

### Using a reverse proxy (nginx + SSL)

```nginx
server {
    listen 443 ssl;
    server_name your-domain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location /webhook {
        proxy_pass http://127.0.0.1:8080/webhook;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

`.env` for webhook:
```env
BOT_MODE=webhook
WEBHOOK_URL=https://your-domain.com
WEBHOOK_PORT=8080
WEBHOOK_PATH=/webhook
WEBHOOK_SECRET=your_random_secret
```

---

## 📚 About tgbotrs

[tgbotrs](https://github.com/ankit-chaubey/tgbotrs) is a fully-featured, auto-generated Telegram Bot API library for Rust.

```toml
[dependencies]
tgbotrs = { version = "0.1.4", features = ["webhook"] }
tokio   = { version = "1", features = ["full"] }
```

**Quick example:**

```rust
use tgbotrs::{Bot, Poller, UpdateHandler};

#[tokio::main]
async fn main() {
    let bot = Bot::new("YOUR_TOKEN").await.unwrap();
    println!("Running as @{}", bot.me.username.as_deref().unwrap_or("unknown"));

    let handler: UpdateHandler = Box::new(|bot, update| {
        Box::pin(async move {
            if let Some(msg) = update.message {
                if let Some(text) = msg.text {
                    let _ = bot.send_message(msg.chat.id, text, None).await;
                }
            }
        })
    });

    Poller::new(bot, handler).timeout(30).start().await.unwrap();
}
```

---

## 👨‍💻 Developer

**Ankit Chaubey**  
📧 [ankitchaubey.dev@gmail.com](mailto:ankitchaubey.dev@gmail.com)  
💬 Telegram: [@ankify](https://t.me/ankify)  
🐙 GitHub: [@ankit-chaubey](https://github.com/ankit-chaubey)

---

## 🔗 Links

| Resource | URL |
|----------|-----|
| 🤖 Bot | [@RustaceBot](https://t.me/RustaceBot) |
| 📦 tgbotrs on crates.io | [crates.io/crates/tgbotrs](https://crates.io/crates/tgbotrs) |
| 📖 Documentation | [docs.rs/tgbotrs](https://docs.rs/tgbotrs) |
| 🐙 tgbotrs Library | [github.com/ankit-chaubey/tgbotrs](https://github.com/ankit-chaubey/tgbotrs) |
| 🤖 Bot Repository | [github.com/ankit-chaubey/RustaceBot](https://github.com/ankit-chaubey/RustaceBot) |
| 🌐 Telegram Bot API | [core.telegram.org/bots/api](https://core.telegram.org/bots/api) |

---

## 📄 License

MIT License — Copyright (c) 2024-present Ankit Chaubey

See [LICENSE](LICENSE) for details.

---

<div align="center">

*Made with 🦀 Rust and ❤️ by Ankit Chaubey*

</div>
