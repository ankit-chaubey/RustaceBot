// ════════════════════════════════════════════════════════════════
//  Rustace Bot — Command Handlers
// ════════════════════════════════════════════════════════════════

use rand::Rng;
use tgbotrs::{
    gen_methods::{
        EditMessageTextParams, GetUserProfilePhotosParams, SendChatActionParams,
        SendContactParams, SendDiceParams, SendLocationParams,
        SendMessageParams, SendPollParams, SendVenueParams,
    },
    types::{
        BotCommand, InlineKeyboardButton, InlineKeyboardMarkup,
        InputPollOption,
    },
    Bot, ChatId, ReplyMarkup,
};

// ── Edit-or-send helper ───────────────────────────────────────────────────────

pub async fn edit_or_send(
    bot: &Bot,
    chat_id: i64,
    message_id: Option<i64>,
    text: &str,
    kb: InlineKeyboardMarkup,
) {
    if let Some(mid) = message_id {
        let params = EditMessageTextParams::new()
            .chat_id(ChatId::from(chat_id))
            .message_id(mid)
            .parse_mode("HTML")
            .reply_markup(kb);
        let _ = bot.edit_message_text(text, Some(params)).await;
    } else {
        let p = SendMessageParams::new()
            .parse_mode("HTML")
            .reply_markup(ReplyMarkup::InlineKeyboard(kb));
        let _ = bot.send_message(chat_id, text, Some(p)).await;
    }
}

// ── Keyboard helpers ──────────────────────────────────────────────────────────

pub fn btn(text: &str, data: &str) -> InlineKeyboardButton {
    InlineKeyboardButton {
        text: text.into(),
        callback_data: Some(data.into()),
        ..Default::default()
    }
}

pub fn link_btn(text: &str, url: &str) -> InlineKeyboardButton {
    InlineKeyboardButton {
        text: text.into(),
        url: Some(url.into()),
        ..Default::default()
    }
}

pub fn main_menu() -> ReplyMarkup {
    ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![btn("🦀 About Rustace", "about"), btn("📚 Library Info", "library")],
            vec![btn("🎮 Fun & Games", "fun_menu"), btn("📡 API Showcase", "api_menu")],
            vec![btn("🛠 Tools", "tools_menu"), btn("📊 Bot Stats", "stats_info")],
            vec![btn("💬 Media Demo", "media_menu"), btn("🔔 Alerts Demo", "alerts_menu")],
            vec![link_btn("🌐 GitHub", "https://github.com/ankit-chaubey/RustaceBot")],
        ],
    })
}

pub fn back_btn() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("⬅️ Main Menu", "main_menu")]],
    }
}

pub fn fun_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![btn("🎲 Roll Dice", "dice"), btn("🎯 Darts", "darts")],
            vec![btn("🎳 Bowling", "bowling"), btn("🏀 Basketball", "basketball")],
            vec![btn("⚽ Football", "football"), btn("🎰 Slot Machine", "slots")],
            vec![btn("💡 Random Fact", "fact"), btn("😂 Joke", "joke")],
            vec![btn("🔮 Magic 8-Ball", "magic8"), btn("🪙 Coin Flip", "coinflip")],
            vec![btn("⬅️ Back", "main_menu")],
        ],
    }
}

pub fn api_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![btn("📋 Webhook Info", "webhook_info"), btn("⭐ Star Balance", "stars")],
            vec![btn("👑 Chat Admins", "admins"), btn("📊 Member Count", "member_count")],
            vec![btn("🔗 Invite Link", "invite_link"), btn("📄 My Commands", "my_commands")],
            vec![btn("👤 My Profile", "my_profile"), btn("🤖 Bot Details", "bot_details")],
            vec![btn("⬅️ Back", "main_menu")],
        ],
    }
}

pub fn tools_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![btn("📍 Location", "location"), btn("📞 Contact", "contact")],
            vec![btn("🏢 Venue", "venue"), btn("📊 Create Poll", "poll")],
            vec![btn("🎯 Checklist Info", "checklist"), btn("⏱️ Live Location", "countdown")],
            vec![btn("🔤 Text Styles", "text_styles"), btn("🌐 Web App Info", "webapp_info")],
            vec![btn("⬅️ Back", "main_menu")],
        ],
    }
}

pub fn media_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![btn("🖼 Send Photo", "send_photo"), btn("🎬 Send Animation", "send_animation")],
            vec![btn("🎵 Audio Info", "audio_info"), btn("📹 Video Info", "video_info")],
            vec![btn("🎤 Voice Info", "voice_info"), btn("📁 Document Info", "doc_info")],
            vec![btn("🎭 Sticker Info", "sticker_info"), btn("📦 Media Group", "media_group_info")],
            vec![btn("⬅️ Back", "main_menu")],
        ],
    }
}

pub fn alerts_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![btn("🚨 Show Alert", "alert_demo"), btn("📢 Notification", "notif_demo")],
            vec![btn("🔗 Callback URL", "cb_url_demo"), btn("💬 Toast", "toast_demo")],
            vec![btn("⬅️ Back", "main_menu")],
        ],
    }
}

// ── /start ────────────────────────────────────────────────────────────────────

pub async fn handle_start(bot: &Bot, chat_id: i64, first_name: &str) {
    let text = format!(
        "🦀 <b>Welcome to Rustace, {}!</b>\n\n\
        I'm <b>@RustaceBot</b> — the official showcase bot for \
        <a href=\"https://github.com/ankit-chaubey/tgbotrs\">tgbotrs</a>, \
        a fully-featured Rust library for the Telegram Bot API.\n\n\
        ✨ <b>What I showcase:</b>\n\
        • All <b>165 API methods</b> from Telegram Bot API 9.4\n\
        • All <b>285 types</b> — fully typed\n\
        • Long-polling via <code>Poller</code>\n\
        • Webhook via <code>WebhookServer</code>\n\
        • Inline keyboards, reply keyboards &amp; more\n\n\
        Built with ❤️ by <a href=\"https://github.com/ankit-chaubey\">Ankit Chaubey</a>\n\
        Library: <a href=\"https://github.com/ankit-chaubey/tgbotrs\">tgbotrs v0.1.4</a>\n\n\
        👇 <b>Explore:</b>",
        first_name
    );

    let params = SendMessageParams::new()
        .parse_mode("HTML")
        .reply_markup(main_menu());

    let _ = bot.send_message(chat_id, text, Some(params)).await;
}

// ── /help ─────────────────────────────────────────────────────────────────────

pub async fn handle_help(bot: &Bot, chat_id: i64) {
    let text = "🦀 <b>Rustace Bot — Command Reference</b>\n\n\
        <b>General</b>\n\
        /start — Welcome &amp; main menu\n\
        /help — This help message\n\
        /about — About Rustace &amp; tgbotrs\n\
        /menu — Show main menu\n\n\
        <b>Fun</b>\n\
        /dice — Roll a dice 🎲\n\
        /darts — Throw darts 🎯\n\
        /bowling — Bowling 🎳\n\
        /basketball — Basketball 🏀\n\
        /football — Football ⚽\n\
        /slots — Slot machine 🎰\n\
        /fact — Random Rust fact 💡\n\
        /joke — Programmer joke 😂\n\
        /magic8 — Magic 8-ball 🔮\n\
        /coinflip — Coin flip 🪙\n\n\
        <b>API Showcase</b>\n\
        /botinfo — Bot information (get_me)\n\
        /webhookinfo — Webhook status\n\
        /membercount — Chat member count\n\
        /admins — List administrators\n\
        /invitelink — Generate invite link\n\
        /mycommands — Show registered commands\n\
        /myprofile — Your profile photos\n\
        /library — Full library overview\n\n\
        <b>Media</b>\n\
        /photo — Demo photo\n\
        /animation — Demo animation\n\
        /location — Share location\n\
        /venue — Share venue\n\
        /contact — Share contact\n\
        /poll — Create poll\n\
        /textstyles — HTML formatting demo\n\n\
        <b>Admin</b>\n\
        /setcommands — Register bot commands\n\
        /deletecommands — Delete commands\n\
        /deletewebhook — Remove webhook\n\
        /stats — Bot statistics\n\
        /ping — Check bot latency 🏓\n\n\
        <i>Source: github.com/ankit-chaubey/RustaceBot</i>";

    let params = SendMessageParams::new()
        .parse_mode("HTML")
        .reply_markup(ReplyMarkup::InlineKeyboard(back_btn()));

    let _ = bot.send_message(chat_id, text, Some(params)).await;
}

// ── /about ────────────────────────────────────────────────────────────────────

pub async fn handle_about(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    let text = "🦀 <b>About Rustace Bot</b>\n\n\
        <b>Bot:</b> @RustaceBot (Rustace)\n\
        <b>Version:</b> 0.1.0\n\
        <b>Library:</b> tgbotrs v0.1.4\n\
        <b>Language:</b> Rust 🦀\n\n\
        <b>🌟 What is tgbotrs?</b>\n\
        tgbotrs is a fully-featured, auto-generated Telegram Bot API library \
        for Rust. All <b>285 types</b> and <b>165 methods</b> from \
        Telegram Bot API 9.4 are strongly typed, fully async, and generated \
        automatically from the official spec.\n\n\
        <b>✨ Features:</b>\n\
        • 165 API methods (all of them!)\n\
        • 285 strongly-typed types\n\
        • Long-polling via <code>Poller</code>\n\
        • Webhook server via <code>WebhookServer</code>\n\
        • File uploads via <code>InputFile</code>\n\
        • Inline &amp; reply keyboards\n\
        • Fully async with tokio\n\n\
        <b>👨‍💻 Developer:</b>\n\
        <a href=\"https://github.com/ankit-chaubey\">Ankit Chaubey</a>\n\
        💬 <a href=\"https://t.me/ankify\">@ankify</a>\n\n\
        <b>🔗 Links:</b>\n\
        • <a href=\"https://github.com/ankit-chaubey/tgbotrs\">tgbotrs library</a>\n\
        • <a href=\"https://github.com/ankit-chaubey/RustaceBot\">Bot source code</a>\n\
        • <a href=\"https://crates.io/crates/tgbotrs\">crates.io/crates/tgbotrs</a>\n\
        • <a href=\"https://docs.rs/tgbotrs\">docs.rs/tgbotrs</a>";

    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![
                link_btn("📦 crates.io", "https://crates.io/crates/tgbotrs"),
                link_btn("📖 docs.rs", "https://docs.rs/tgbotrs"),
            ],
            vec![
                link_btn("🐙 Library", "https://github.com/ankit-chaubey/tgbotrs"),
                link_btn("🤖 Bot Repo", "https://github.com/ankit-chaubey/RustaceBot"),
            ],
            vec![btn("⬅️ Main Menu", "main_menu")],
        ],
    };
    edit_or_send(bot, chat_id, message_id, text, kb).await;
}

// ── Dice variants ─────────────────────────────────────────────────────────────

pub async fn handle_dice(bot: &Bot, chat_id: i64, emoji: &str) {
    let _ = bot.send_chat_action(chat_id, "typing", Some(SendChatActionParams::new())).await;

    let params = SendDiceParams::new().emoji(emoji.to_string());
    let _ = bot.send_dice(chat_id, Some(params)).await;

    let label = match emoji {
        "🎯" => "Darts! 🎯",
        "🎳" => "Bowling! 🎳",
        "🏀" => "Basketball! 🏀",
        "⚽" => "Football! ⚽",
        "🎰" => "Slot Machine! 🎰",
        _ => "Dice rolled! 🎲",
    };

    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("🎮 More Fun", "fun_menu"), btn("⬅️ Menu", "main_menu")]],
    };
    let p = SendMessageParams::new()
        .parse_mode("HTML")
        .reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, format!("🎉 <b>{}</b> Good luck!", label), Some(p)).await;
}

// ── Facts & Jokes ─────────────────────────────────────────────────────────────

const RUST_FACTS: &[&str] = &[
    "🦀 Rust was voted the <b>most loved programming language</b> on Stack Overflow for 9 years in a row (2016–2024)!",
    "🔒 Rust's <b>borrow checker</b> eliminates entire classes of bugs like null pointer dereferences and data races at compile time.",
    "⚡ Rust has <b>zero-cost abstractions</b> — high-level code compiles to the same machine code as hand-written low-level C code.",
    "📦 The Rust package manager, <b>Cargo</b>, is consistently praised as one of the best build systems in the programming world.",
    "🌐 <b>Mozilla</b> originally created Rust to write a safe browser engine. That engine, Servo, influenced Firefox's rendering.",
    "🏗️ <b>Linux kernel</b> officially supports Rust since version 6.1, making it the first new language added in 30+ years.",
    "🔬 Rust has no <b>garbage collector</b> — memory is managed through ownership and lifetimes, giving C-like performance.",
    "💼 Companies like <b>Microsoft, Google, Meta, Cloudflare, Amazon</b> all use Rust in production systems.",
    "🎓 The Rust compiler error messages are so good, many developers say they <b>learn Rust from compiler errors</b>.",
    "🤖 <b>tgbotrs</b> — this bot's library — covers all 285 types and 165 methods of Telegram Bot API 9.4 in pure Rust!",
];

const JOKES: &[&str] = &[
    "Why do Rust developers never get into fights?\n\nBecause their <b>borrow checker</b> prevents them from taking things that aren't theirs! 😄",
    "How many Rust programmers does it take to change a light bulb?\n\nNone — the compiler told them the bulb's <b>lifetime</b> is still valid! 💡",
    "A C++ developer and a Rust developer walk into a bar.\n\nThe C++ developer has a <b>segfault</b>. The Rust developer just smiles. 🦀",
    "Why did the Rust developer break up with Python?\n\nToo many <b>runtime errors</b>. Rust only does heartbreak at compile time. 💔",
    "How do you know someone uses Rust?\n\nDon't worry — the <b>borrow checker</b> will tell you. 😂",
    "What's a Rust programmer's favorite restaurant order?\n\n<b>Zero-cost abstraction</b> — looks expensive, tastes free! 🍽️",
    "Why did the Rust program get promoted?\n\nBecause it had <b>no memory leaks</b> and excellent <b>ownership</b> skills! 📈",
    "Interviewer: 'Do you know C++?' \nRust dev: 'I used to, but then I <b>moved</b>.' 😎",
];

// ── Ping ─────────────────────────────────────────────────────────────────────

pub async fn handle_ping(bot: &Bot, chat_id: i64, msg_date: i64) {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Message delay: difference between when Telegram stamped the message
    // and when our bot started processing it (both in Unix seconds).
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let msg_delay_ms = (now_secs - msg_date).max(0) * 1000;

    // Send initial "measuring" message and time the round-trip.
    let rtt_start = tokio::time::Instant::now();
    let sent = bot.send_message(chat_id, "🏓 Pong! Measuring...", None).await;
    let rtt_ms = rtt_start.elapsed().as_millis();

    if let Ok(sent_msg) = sent {
        let text = format!(
            "🏓 <b>Pong!</b>\n\n\
            ⚡ <b>API RTT:</b>    <code>{rtt_ms} ms</code>\n\
            📨 <b>Msg Delay:</b>  <code>{msg_delay_ms} ms</code>\n\n\
            <i>RTT = time for bot→Telegram→bot round-trip\n\
            Delay = time message spent before bot processed it</i>"
        );
        let edit_params = tgbotrs::gen_methods::EditMessageTextParams::new()
            .chat_id(tgbotrs::ChatId::from(chat_id))
            .message_id(sent_msg.message_id)
            .parse_mode("HTML");
        let _ = bot.edit_message_text(&text, Some(edit_params)).await;
    }
}

pub async fn handle_fact(bot: &Bot, chat_id: i64) {
    let fact = {
        let mut rng = rand::thread_rng();
        RUST_FACTS[rng.gen_range(0..RUST_FACTS.len())]
    };
    let text = format!("💡 <b>Random Rust / tgbotrs Fact</b>\n\n{}", fact);
    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("💡 Another Fact", "fact"), btn("⬅️ Menu", "main_menu")]],
    };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

pub async fn handle_joke(bot: &Bot, chat_id: i64) {
    let joke = {
        let mut rng = rand::thread_rng();
        JOKES[rng.gen_range(0..JOKES.len())]
    };
    let text = format!("😂 <b>Programmer Joke</b>\n\n{}", joke);
    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("😂 Another Joke", "joke"), btn("⬅️ Menu", "main_menu")]],
    };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

const EIGHT_BALL: &[&str] = &[
    "✅ It is certain.", "✅ It is decidedly so.", "✅ Without a doubt.",
    "✅ Yes, definitely.", "✅ You may rely on it.", "✅ As I see it, yes.",
    "✅ Most likely.", "✅ Outlook good.", "✅ Yes.", "✅ Signs point to yes.",
    "🌫️ Reply hazy, try again.", "🌫️ Ask again later.", "🌫️ Better not tell you now.",
    "🌫️ Cannot predict now.", "🌫️ Concentrate and ask again.",
    "❌ Don't count on it.", "❌ My reply is no.", "❌ My sources say no.",
    "❌ Outlook not so good.", "❌ Very doubtful.",
];

pub async fn handle_magic8(bot: &Bot, chat_id: i64) {
    let answer = {
        let mut rng = rand::thread_rng();
        EIGHT_BALL[rng.gen_range(0..EIGHT_BALL.len())]
    };
    let text = format!("🔮 <b>Magic 8-Ball</b>\n\n<i>The spirits say...</i>\n\n<b>{}</b>", answer);
    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("🔮 Ask Again", "magic8"), btn("⬅️ Menu", "main_menu")]],
    };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

pub async fn handle_coinflip(bot: &Bot, chat_id: i64) {
    let result = {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) { "🪙 <b>HEADS!</b>" } else { "🪙 <b>TAILS!</b>" }
    };
    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("🪙 Flip Again", "coinflip"), btn("⬅️ Menu", "main_menu")]],
    };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, format!("{} The coin has spoken!", result), Some(p)).await;
}

// ── /photo ────────────────────────────────────────────────────────────────────

pub async fn handle_photo(bot: &Bot, chat_id: i64) {
    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![
            btn("🎬 Animation", "send_animation"),
            btn("⬅️ Menu", "main_menu"),
        ]],
    };
    let p = SendMessageParams::new()
        .parse_mode("HTML")
        .reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot
        .send_message(
            chat_id,
            "📷 <b>send_photo demo</b>\n\n\
            The photo method supports:\n\
            • <code>file_id</code> — re-use uploaded files\n\
            • <code>URL</code> — link to an image\n\
            • <code>InputFile</code> — local file upload\n\n\
            <b>Optional params:</b> caption, parse_mode, has_spoiler, reply_markup\n\n\
            <pre>bot.send_photo(chat_id, file_or_url, Some(params)).await</pre>",
            Some(p),
        )
        .await;
}

// ── /animation ────────────────────────────────────────────────────────────────

pub async fn handle_animation(bot: &Bot, chat_id: i64) {
    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![
            btn("🖼 Photo", "send_photo"),
            btn("⬅️ Menu", "main_menu"),
        ]],
    };
    let p = SendMessageParams::new()
        .parse_mode("HTML")
        .reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot
        .send_message(
            chat_id,
            "🎬 <b>send_animation demo</b>\n\n\
            Sends GIF/MP4 animations.\n\n\
            <b>Optional params:</b>\n\
            • <code>caption</code>, <code>parse_mode</code>\n\
            • <code>duration</code>, <code>width</code>, <code>height</code>\n\
            • <code>thumbnail</code>\n\
            • <code>has_spoiler</code>\n\n\
            <pre>bot.send_animation(chat_id, file, Some(params)).await</pre>",
            Some(p),
        )
        .await;
}

// ── /location ─────────────────────────────────────────────────────────────────

pub async fn handle_location(bot: &Bot, chat_id: i64) {
    let params = SendLocationParams::new();
    let _ = bot.send_location(chat_id, 48.8584, 2.2945, Some(params)).await;

    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("🏢 Venue", "venue"), btn("⬅️ Menu", "main_menu")]],
    };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id,
        "📍 <b>Location sent!</b>\n\nEiffel Tower, Paris 🗼\n\
        <code>bot.send_location(chat_id, lat, lon, params)</code>", Some(p)).await;
}

// ── /venue ────────────────────────────────────────────────────────────────────

pub async fn handle_venue(bot: &Bot, chat_id: i64) {
    let params = SendVenueParams::new().foursquare_id("4ADCDA06F964A520B4B61FE3".to_string());
    let _ = bot.send_venue(chat_id, 48.8584, 2.2945,
        "Eiffel Tower 🗼", "Champ de Mars, 75007 Paris", Some(params)).await;

    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("📞 Contact", "contact"), btn("⬅️ Menu", "main_menu")]],
    };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id,
        "🏢 <b>Venue sent!</b>\n\n\
        <code>bot.send_venue(chat_id, lat, lon, title, address, params)</code>", Some(p)).await;
}

// ── /contact ──────────────────────────────────────────────────────────────────

pub async fn handle_contact(bot: &Bot, chat_id: i64) {
    let params = SendContactParams::new().last_name("Chaubey".to_string());
    let _ = bot.send_contact(chat_id, "+1234567890", "Ankit", Some(params)).await;

    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("📊 Poll", "poll"), btn("⬅️ Menu", "main_menu")]],
    };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id,
        "📞 <b>Contact sent!</b>\n\n\
        <code>bot.send_contact(chat_id, phone, first_name, params)</code>", Some(p)).await;
}

// ── /poll ─────────────────────────────────────────────────────────────────────

pub async fn handle_poll(bot: &Bot, chat_id: i64) {
    let params = SendPollParams::new()
        .is_anonymous(false)
        .allows_multiple_answers(true);
    let _ = bot.send_poll(chat_id,
        "🦀 What do you love most about Rust?",
        vec![
            InputPollOption { text: "🔒 Memory Safety".to_string(), text_parse_mode: None, text_entities: None },
            InputPollOption { text: "⚡ Performance".to_string(), text_parse_mode: None, text_entities: None },
            InputPollOption { text: "🦺 Type System".to_string(), text_parse_mode: None, text_entities: None },
            InputPollOption { text: "📦 Cargo".to_string(), text_parse_mode: None, text_entities: None },
            InputPollOption { text: "😊 Community".to_string(), text_parse_mode: None, text_entities: None },
        ],
        Some(params)).await;

    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("⬅️ Menu", "main_menu")]],
    };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id,
        "📊 <b>Poll created!</b>\n\
        <code>bot.send_poll(chat_id, question, options, params)</code>", Some(p)).await;
}

// ── /textstyles ───────────────────────────────────────────────────────────────

pub async fn handle_text_styles(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    let text = "✨ <b>HTML Text Formatting Showcase</b>\n\n\
        <b>Bold text</b>\n\
        <i>Italic text</i>\n\
        <u>Underlined text</u>\n\
        <s>Strikethrough text</s>\n\
        <code>Monospace / inline code</code>\n\
        <pre>Pre-formatted block\nMultiple lines</pre>\n\
        <tg-spoiler>Hidden spoiler text</tg-spoiler>\n\
        <a href=\"https://github.com/ankit-chaubey/tgbotrs\">Link text</a>\n\n\
        <b>Nested:</b> <b><i>Bold italic</i></b> | <i><code>italic code</code></i>\n\n\
        <blockquote>This is a blockquote.</blockquote>\n\n\
        <i>All via <code>parse_mode = HTML</code> in SendMessageParams</i>";

    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("⬅️ Menu", "main_menu")]],
    };
    edit_or_send(bot, chat_id, message_id, text, kb).await;
}

// ── /botinfo ──────────────────────────────────────────────────────────────────

pub async fn handle_bot_info(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    match bot.get_me().await {
        Ok(me) => {
            let text = format!(
                "🤖 <b>Bot Information</b> <i>(get_me)</i>\n\n\
                <b>ID:</b> <code>{}</code>\n\
                <b>Name:</b> {}\n\
                <b>Username:</b> @{}\n\
                <b>Is Bot:</b> {}\n\
                <b>Can Join Groups:</b> {}\n\
                <b>Supports Inline:</b> {}\n\
                <b>Can Connect to Business:</b> {}\n\
                <b>Has Main Web App:</b> {}",
                me.id,
                me.first_name,
                me.username.as_deref().unwrap_or("unknown"),
                me.is_bot,
                me.can_join_groups.unwrap_or(false),
                me.supports_inline_queries.unwrap_or(false),
                me.can_connect_to_business.unwrap_or(false),
                me.has_main_web_app.unwrap_or(false),
            );
            let kb = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![btn("⬅️ Menu", "main_menu")]],
            };
            edit_or_send(bot, chat_id, message_id, &text, kb).await;
        }
        Err(e) => {
            edit_or_send(bot, chat_id, message_id, &format!("❌ get_me failed: {}", e), back_btn()).await;
        }
    }
}

// ── Webhook info ──────────────────────────────────────────────────────────────

pub async fn handle_webhook_info(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    match bot.get_webhook_info().await {
        Ok(info) => {
            let text = format!(
                "📡 <b>Webhook Info</b>\n\n\
                <b>URL:</b> <code>{}</code>\n\
                <b>Custom Cert:</b> {}\n\
                <b>Pending Updates:</b> {}\n\
                <b>Last Error:</b> {}\n\
                <b>Max Connections:</b> {}\n\
                <b>Allowed Updates:</b> {}",
                if info.url.is_empty() { "None (polling mode)".to_string() } else { info.url.clone() },
                info.has_custom_certificate,
                info.pending_update_count,
                info.last_error_message.as_deref().unwrap_or("None"),
                info.max_connections.unwrap_or(0),
                info.allowed_updates.as_ref().map_or("All".into(), |u| u.join(", ")),
            );
            let kb = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![btn("⬅️ API Menu", "api_menu")]],
            };
            edit_or_send(bot, chat_id, message_id, &text, kb).await;
        }
        Err(e) => {
            edit_or_send(bot, chat_id, message_id, &format!("❌ Error: {}", e), back_btn()).await;
        }
    }
}

// ── Member count ──────────────────────────────────────────────────────────────

pub async fn handle_member_count(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    match bot.get_chat_member_count(chat_id).await {
        Ok(count) => {
            let text = format!(
                "👥 <b>Chat Member Count</b>\n\n\
                This chat has <b>{}</b> member(s).\n\n\
                <code>bot.get_chat_member_count(chat_id)</code>",
                count
            );
            let kb = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![btn("⬅️ API Menu", "api_menu")]],
            };
            edit_or_send(bot, chat_id, message_id, &text, kb).await;
        }
        Err(e) => {
            edit_or_send(bot, chat_id, message_id, &format!("❌ Error: {}", e), back_btn()).await;
        }
    }
}

// ── Chat admins ───────────────────────────────────────────────────────────────

pub async fn handle_admins(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    match bot.get_chat_administrators(chat_id).await {
        Ok(admins) => {
            let admin_list: Vec<String> = admins.iter().map(|a| {
                let v = serde_json::to_value(a).unwrap_or_default();
                let name = v.get("user").and_then(|u| u.get("first_name")).and_then(|n| n.as_str()).unwrap_or("Unknown");
                let uname = v.get("user").and_then(|u| u.get("username")).and_then(|n| n.as_str())
                    .map(|u| format!(" (@{})", u)).unwrap_or_default();
                format!("• {}{}", name, uname)
            }).collect();

            let text = format!(
                "👑 <b>Chat Administrators</b> ({} total)\n\n{}\n\n\
                <code>bot.get_chat_administrators(chat_id)</code>",
                admins.len(),
                if admin_list.is_empty() { "No admins found.".to_string() } else { admin_list.join("\n") }
            );
            let kb = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![btn("⬅️ API Menu", "api_menu")]],
            };
            edit_or_send(bot, chat_id, message_id, &text, kb).await;
        }
        Err(e) => {
            let kb = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![btn("⬅️ API Menu", "api_menu")]],
            };
            edit_or_send(bot, chat_id, message_id,
                &format!("⚠️ <b>get_chat_administrators</b>\n\nOnly works in groups.\nError: {}", e), kb).await;
        }
    }
}

// ── Invite link ───────────────────────────────────────────────────────────────

pub async fn handle_invite_link(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    match bot.export_chat_invite_link(chat_id).await {
        Ok(link) => {
            let text = format!(
                "🔗 <b>Chat Invite Link</b>\n\n<code>{}</code>\n\n\
                Via <code>bot.export_chat_invite_link(chat_id)</code>",
                link
            );
            let kb = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![
                    link_btn("🔗 Join", &link),
                    btn("⬅️ API Menu", "api_menu"),
                ]],
            };
            edit_or_send(bot, chat_id, message_id, &text, kb).await;
        }
        Err(e) => {
            let kb = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![btn("⬅️ API Menu", "api_menu")]],
            };
            edit_or_send(bot, chat_id, message_id,
                &format!("⚠️ Only works for groups/channels where bot is admin.\nError: {}", e), kb).await;
        }
    }
}

// ── My commands ───────────────────────────────────────────────────────────────

pub async fn handle_my_commands(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    match bot.get_my_commands(None).await {
        Ok(cmds) => {
            let list: Vec<String> = cmds.iter()
                .map(|c| format!("/{} — {}", c.command, c.description))
                .collect();
            let text = format!(
                "📋 <b>Registered Commands</b> ({} total)\n\n{}\n\n\
                Via <code>bot.get_my_commands()</code>",
                cmds.len(),
                if list.is_empty() { "None registered.".into() } else { list.join("\n") }
            );
            let kb = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![btn("⬅️ API Menu", "api_menu")]],
            };
            edit_or_send(bot, chat_id, message_id, &text, kb).await;
        }
        Err(e) => {
            edit_or_send(bot, chat_id, message_id, &format!("❌ Error: {}", e), back_btn()).await;
        }
    }
}

// ── My profile ────────────────────────────────────────────────────────────────

pub async fn handle_my_profile(bot: &Bot, chat_id: i64, user_id: i64) {
    let params = GetUserProfilePhotosParams::new().limit(5_i64);
    match bot.get_user_profile_photos(user_id, Some(params)).await {
        Ok(photos) => {
            let text = format!(
                "👤 <b>Your Profile Photos</b>\n\n\
                Total: <b>{}</b>\n\
                Fetched: <b>{}</b>\n\n\
                <code>bot.get_user_profile_photos(user_id, params)</code>",
                photos.total_count,
                photos.photos.len()
            );
            let kb = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![btn("⬅️ API Menu", "api_menu")]],
            };
            let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
            let _ = bot.send_message(chat_id, text, Some(p)).await;
        }
        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Error: {}", e), None).await; }
    }
}

// ── Library info ──────────────────────────────────────────────────────────────

pub async fn handle_library(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    let text = "📚 <b>tgbotrs — Library Overview</b>\n\n\
        <b>✅ 165 Methods across 12 categories:</b>\n\n\
        <b>📨 Messaging (17)</b>\n\
        <code>send_message, send_photo, send_audio, send_document,\
        send_video, send_animation, send_voice, send_video_note,\
        send_sticker, send_location, send_venue, send_contact,\
        send_dice, send_poll, send_game, send_invoice, send_paid_media</code>\n\n\
        <b>✏️ Editing (7)</b>\n\
        <code>edit_message_text, edit_message_caption, edit_message_media,\
        edit_message_reply_markup, edit_message_live_location,\
        stop_message_live_location, edit_message_checklist</code>\n\n\
        <b>💬 Chat Management (23)</b>\n\
        <code>get_chat, get_chat_administrators, ban_chat_member,\
        unban_chat_member, restrict_chat_member, promote_chat_member,\
        pin_chat_message, unpin_chat_message, leave_chat...</code>\n\n\
        <b>🎭 Stickers (15)</b>\n\
        <code>get_sticker_set, create_new_sticker_set, add_sticker_to_set,\
        delete_sticker_from_set, set_sticker_emoji_list...</code>\n\n\
        <b>🔍 Inline (3)</b>\n\
        <code>answer_inline_query, answer_web_app_query, save_prepared_inline_message</code>\n\n\
        <b>💳 Payments &amp; Stars (10)</b>\n\
        <code>send_invoice, answer_shipping_query, answer_pre_checkout_query,\
        get_star_transactions, get_my_star_balance, gift_premium_subscription...</code>\n\n\
        <b>🏢 Business (12)</b>\n\
        <code>get_business_connection, set_business_account_name,\
        set_business_account_bio, read_business_message...</code>\n\n\
        <b>🎮 Games (3)</b>\n\
        <code>send_game, set_game_score, get_game_high_scores</code>\n\n\
        <b>📢 Stories (5)</b>\n\
        <code>post_story, edit_story, delete_story, repost_story, approve_suggested_post</code>\n\n\
        <b>⚙️ Settings (13)</b>\n\
        <code>set_my_commands, get_my_commands, delete_my_commands,\
        set_my_name, set_my_description, set_my_short_description...</code>\n\n\
        <i>All methods fully async, strongly typed — tgbotrs v0.1.4</i>";

    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![
                link_btn("📦 crates.io", "https://crates.io/crates/tgbotrs"),
                link_btn("📖 docs.rs", "https://docs.rs/tgbotrs"),
            ],
            vec![btn("⬅️ Main Menu", "main_menu")],
        ],
    };
    edit_or_send(bot, chat_id, message_id, text, kb).await;
}

// ── Media info cards ──────────────────────────────────────────────────────────

pub async fn handle_audio_info(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    let text = "🎵 <b>send_audio</b>\n\n\
        Sends audio files (MP3, FLAC, OGG, M4A).\n\n\
        <b>Optional params:</b> caption, duration, performer, title, thumbnail\n\n\
        <pre>bot.send_audio(chat_id, file, Some(params)).await</pre>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Media Menu", "media_menu")]] };
    edit_or_send(bot, chat_id, message_id, text, kb).await;
}

pub async fn handle_video_info(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    let text = "📹 <b>send_video</b>\n\n\
        Sends video files (MP4, MOV).\n\n\
        <b>Optional params:</b> duration, width, height, thumbnail, has_spoiler, supports_streaming\n\
        Also: <code>send_video_note()</code> for circular video messages\n\n\
        <pre>bot.send_video(chat_id, file, Some(params)).await</pre>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Media Menu", "media_menu")]] };
    edit_or_send(bot, chat_id, message_id, text, kb).await;
}

pub async fn handle_voice_info(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    let text = "🎤 <b>send_voice</b>\n\n\
        Sends voice messages (OGG/OPUS). Displays as waveform in Telegram.\n\n\
        <b>Optional params:</b> caption, duration\n\n\
        <pre>bot.send_voice(chat_id, ogg_file, Some(params)).await</pre>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Media Menu", "media_menu")]] };
    edit_or_send(bot, chat_id, message_id, text, kb).await;
}

pub async fn handle_doc_info(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    let text = "📁 <b>send_document</b>\n\n\
        Sends any file as a document (PDF, ZIP, source code, etc.).\n\n\
        <b>Optional params:</b> thumbnail, caption, disable_content_type_detection\n\n\
        <pre>bot.send_document(chat_id, file, Some(params)).await</pre>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Media Menu", "media_menu")]] };
    edit_or_send(bot, chat_id, message_id, text, kb).await;
}

pub async fn handle_sticker_info(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    let text = "🎭 <b>Sticker API — 15 Methods</b>\n\n\
        <code>send_sticker</code> — Send sticker by file_id\n\
        <code>get_sticker_set</code> — Fetch sticker pack\n\
        <code>create_new_sticker_set</code> — Create new pack\n\
        <code>add_sticker_to_set</code> — Add to pack\n\
        <code>set_sticker_emoji_list</code> — Set emojis\n\
        <code>set_sticker_keywords</code> — Set search keywords\n\
        <code>set_sticker_mask_position</code> — Face positioning\n\
        <code>replace_sticker_in_set</code> — Replace sticker\n\
        <code>delete_sticker_set</code> — Delete entire pack\n\
        <i>...and 6 more!</i>\n\n\
        <i>Sticker types: static, animated, video</i>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Media Menu", "media_menu")]] };
    edit_or_send(bot, chat_id, message_id, text, kb).await;
}

pub async fn handle_media_group_info(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    let text = "📦 <b>send_media_group</b>\n\n\
        Sends 2–10 items as an album.\n\n\
        <b>InputMedia types:</b>\n\
        • <code>InputMedia::Photo</code>\n\
        • <code>InputMedia::Video</code>\n\
        • <code>InputMedia::Audio</code>\n\
        • <code>InputMedia::Document</code>\n\
        • <code>InputMedia::Animation</code>\n\n\
        <pre>bot.send_media_group(\n  chat_id,\n  vec![InputMedia::Photo(...)],\n  None\n).await</pre>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Media Menu", "media_menu")]] };
    edit_or_send(bot, chat_id, message_id, text, kb).await;
}

pub async fn handle_webapp_info(bot: &Bot, chat_id: i64, message_id: Option<i64>) {
    let text = "🌐 <b>Web App Support</b>\n\n\
        tgbotrs supports Telegram Web Apps:\n\n\
        <b>Inline Button:</b>\n\
        <code>InlineKeyboardButton { web_app: Some(WebAppInfo { url }) }</code>\n\n\
        <b>API method:</b>\n\
        <code>bot.answer_web_app_query(web_app_query_id, result)</code>\n\n\
        <b>Save for later:</b>\n\
        <code>bot.save_prepared_inline_message(user_id, result, params)</code>\n\n\
        <i>Web apps open in an in-app browser within Telegram.</i>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Tools", "tools_menu")]] };
    edit_or_send(bot, chat_id, message_id, text, kb).await;
}

// ── Register commands ─────────────────────────────────────────────────────────

pub async fn register_commands(bot: &Bot) -> Result<(), tgbotrs::BotError> {
    let commands = vec![
        BotCommand { command: "start".into(), description: "🦀 Welcome & main menu".into() },
        BotCommand { command: "help".into(), description: "📖 Show all commands".into() },
        BotCommand { command: "about".into(), description: "ℹ️ About Rustace & tgbotrs".into() },
        BotCommand { command: "menu".into(), description: "📋 Show main menu".into() },
        BotCommand { command: "dice".into(), description: "🎲 Roll a dice".into() },
        BotCommand { command: "darts".into(), description: "🎯 Throw darts".into() },
        BotCommand { command: "bowling".into(), description: "🎳 Play bowling".into() },
        BotCommand { command: "basketball".into(), description: "🏀 Shoot hoops".into() },
        BotCommand { command: "football".into(), description: "⚽ Kick the ball".into() },
        BotCommand { command: "slots".into(), description: "🎰 Slot machine".into() },
        BotCommand { command: "fact".into(), description: "💡 Random Rust fact".into() },
        BotCommand { command: "joke".into(), description: "😂 Programmer joke".into() },
        BotCommand { command: "magic8".into(), description: "🔮 Magic 8-ball".into() },
        BotCommand { command: "coinflip".into(), description: "🪙 Flip a coin".into() },
        BotCommand { command: "photo".into(), description: "🖼 Demo send_photo".into() },
        BotCommand { command: "animation".into(), description: "🎬 Demo send_animation".into() },
        BotCommand { command: "location".into(), description: "📍 Demo send_location".into() },
        BotCommand { command: "venue".into(), description: "🏢 Demo send_venue".into() },
        BotCommand { command: "contact".into(), description: "📞 Demo send_contact".into() },
        BotCommand { command: "poll".into(), description: "📊 Create a poll".into() },
        BotCommand { command: "textstyles".into(), description: "✨ HTML formatting demo".into() },
        BotCommand { command: "botinfo".into(), description: "🤖 Bot info (get_me)".into() },
        BotCommand { command: "webhookinfo".into(), description: "📡 Webhook status".into() },
        BotCommand { command: "membercount".into(), description: "👥 Member count".into() },
        BotCommand { command: "admins".into(), description: "👑 List admins".into() },
        BotCommand { command: "invitelink".into(), description: "🔗 Get invite link".into() },
        BotCommand { command: "mycommands".into(), description: "📋 Show commands".into() },
        BotCommand { command: "myprofile".into(), description: "👤 Profile photos".into() },
        BotCommand { command: "library".into(), description: "📚 Library overview".into() },
        BotCommand { command: "stats".into(), description: "📊 Bot statistics".into() },
        BotCommand { command: "ping".into(),  description: "🏓 Check bot latency & response time".into() },
        BotCommand { command: "setcommands".into(), description: "⚙️ Register commands".into() },
        BotCommand { command: "deletecommands".into(), description: "🗑 Delete commands".into() },
        // ── Moderation ────────────────────────────────────────────────────────
        BotCommand { command: "modhelp".into(), description: "🛡️ Moderation help".into() },
        BotCommand { command: "ban".into(), description: "🔨 Ban user (reply) [duration]".into() },
        BotCommand { command: "unban".into(), description: "✅ Unban user (reply)".into() },
        BotCommand { command: "kick".into(), description: "👢 Kick user (reply)".into() },
        BotCommand { command: "mute".into(), description: "🔇 Mute user (reply) [duration]".into() },
        BotCommand { command: "unmute".into(), description: "🔊 Unmute user (reply)".into() },
        BotCommand { command: "warn".into(), description: "⚠️ Warn user — 3 warns = auto-ban".into() },
        BotCommand { command: "unwarn".into(), description: "✅ Remove a warning (reply)".into() },
        BotCommand { command: "warns".into(), description: "📋 Check warnings (reply)".into() },
        BotCommand { command: "delete".into(), description: "🗑 Delete replied message".into() },
        BotCommand { command: "pin".into(), description: "📌 Pin replied message".into() },
        BotCommand { command: "unpin".into(), description: "📌 Unpin current message".into() },
        BotCommand { command: "ro".into(), description: "🔇 Read-only mode ON".into() },
        BotCommand { command: "unro".into(), description: "🔊 Read-only mode OFF".into() },
        // ── Admin ─────────────────────────────────────────────────────────────
        BotCommand { command: "promote".into(), description: "⭐ Promote user [reply/id] [Title]".into() },
        BotCommand { command: "demote".into(), description: "🔽 Demote user [reply/id]".into() },
        BotCommand { command: "title".into(), description: "🏷️ Set admin title [reply/id] Title".into() },
        BotCommand { command: "userinfo".into(), description: "👤 User info [reply/id/@user]".into() },
        BotCommand { command: "whois".into(), description: "🔍 Same as /userinfo".into() },
        // ── Filters ───────────────────────────────────────────────────────────
        BotCommand { command: "filter".into(), description: "🔑 Add keyword auto-reply".into() },
        BotCommand { command: "delfilter".into(), description: "🗑 Delete a filter".into() },
        BotCommand { command: "filters".into(), description: "📋 List all active filters".into() },
        // ── Notes ─────────────────────────────────────────────────────────────
        BotCommand { command: "note".into(), description: "📝 Save a note".into() },
        BotCommand { command: "get".into(), description: "📌 Get a saved note".into() },
        BotCommand { command: "notes".into(), description: "📋 List all saved notes".into() },
        BotCommand { command: "delnote".into(), description: "🗑 Delete a note".into() },
        // ── Send / Post / Media ───────────────────────────────────────────────
        BotCommand { command: "send".into(), description: "📨 Send message with inline buttons".into() },
        BotCommand { command: "post".into(), description: "📢 Styled broadcast with buttons".into() },
        BotCommand { command: "img".into(), description: "🖼 Send photo from URL".into() },
        BotCommand { command: "vid".into(), description: "🎬 Send video from URL".into() },
        BotCommand { command: "aud".into(), description: "🎵 Send audio from URL".into() },
        BotCommand { command: "doc".into(), description: "📁 Send document from URL".into() },
        BotCommand { command: "buttons".into(), description: "🎨 Colourful button showcase".into() },
        BotCommand { command: "sendhelp".into(), description: "📡 /send and /post guide".into() },
    ];

    bot.set_my_commands(commands, None).await?;
    log::info!("✅ Bot commands registered successfully");
    Ok(())
}
