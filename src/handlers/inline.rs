// ════════════════════════════════════════════════════════════════
//  Rustace Bot — Inline Query Handler
//  Search tgbotrs methods, types, and Rust facts inline
// ════════════════════════════════════════════════════════════════

use tgbotrs::{
    gen_methods::AnswerInlineQueryParams,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResult,
        InlineQueryResultArticle, InlineQueryResultsButton, InputMessageContent,
        InputTextMessageContent,
    },
    Bot,
};

// ── Article builder ───────────────────────────────────────────────────────────

fn article(id: &str, title: &str, description: &str, text: &str) -> InlineQueryResult {
    InlineQueryResult::InlineQueryResultArticle(InlineQueryResultArticle {
        r#type: "article".to_string(),
        id: id.to_string(),
        title: title.to_string(),
        input_message_content: InputMessageContent::InputTextMessageContent(InputTextMessageContent {
            message_text: text.to_string(),
            parse_mode: Some("HTML".to_string()),
            entities: None,
            link_preview_options: None,
        }),
        reply_markup: Some(Box::new(InlineKeyboardMarkup {
            inline_keyboard: vec![vec![InlineKeyboardButton {
                text: "🦀 tgbotrs Library".into(),
                url: Some("https://github.com/ankit-chaubey/tgbotrs".into()),
                ..Default::default()
            }]],
        })),
        url: Some("https://github.com/ankit-chaubey/tgbotrs".to_string()),
        description: Some(description.to_string()),
        thumbnail_url: None,
        thumbnail_width: None,
        thumbnail_height: None,
    })
}

// ── tgbotrs method database ───────────────────────────────────────────────────

struct Method {
    id: &'static str,
    name: &'static str,
    category: &'static str,
    description: &'static str,
    signature: &'static str,
}

const METHODS: &[Method] = &[
    // Messaging
    Method { id: "m_send_message", name: "send_message", category: "📨 Messaging",
        description: "Send a text message",
        signature: "bot.send_message(chat_id, text, params)" },
    Method { id: "m_send_photo", name: "send_photo", category: "📨 Messaging",
        description: "Send a photo (URL, file_id, or upload)",
        signature: "bot.send_photo(chat_id, photo, params)" },
    Method { id: "m_send_video", name: "send_video", category: "📨 Messaging",
        description: "Send a video file",
        signature: "bot.send_video(chat_id, video, params)" },
    Method { id: "m_send_audio", name: "send_audio", category: "📨 Messaging",
        description: "Send an audio file",
        signature: "bot.send_audio(chat_id, audio, params)" },
    Method { id: "m_send_document", name: "send_document", category: "📨 Messaging",
        description: "Send any file as a document",
        signature: "bot.send_document(chat_id, document, params)" },
    Method { id: "m_send_animation", name: "send_animation", category: "📨 Messaging",
        description: "Send GIF or MP4 animation",
        signature: "bot.send_animation(chat_id, animation, params)" },
    Method { id: "m_send_voice", name: "send_voice", category: "📨 Messaging",
        description: "Send a voice note (OGG/OPUS)",
        signature: "bot.send_voice(chat_id, voice, params)" },
    Method { id: "m_send_sticker", name: "send_sticker", category: "📨 Messaging",
        description: "Send a sticker by file_id",
        signature: "bot.send_sticker(chat_id, sticker, params)" },
    Method { id: "m_send_dice", name: "send_dice", category: "📨 Messaging",
        description: "Send animated emoji dice",
        signature: "bot.send_dice(chat_id, params)" },
    Method { id: "m_send_poll", name: "send_poll", category: "📨 Messaging",
        description: "Send a poll with options",
        signature: "bot.send_poll(chat_id, question, options, params)" },
    Method { id: "m_send_location", name: "send_location", category: "📨 Messaging",
        description: "Send a map location",
        signature: "bot.send_location(chat_id, latitude, longitude, params)" },
    Method { id: "m_send_contact", name: "send_contact", category: "📨 Messaging",
        description: "Send a phone contact",
        signature: "bot.send_contact(chat_id, phone_number, first_name, params)" },
    Method { id: "m_send_venue", name: "send_venue", category: "📨 Messaging",
        description: "Send a venue (location + title + address)",
        signature: "bot.send_venue(chat_id, lat, lon, title, address, params)" },
    Method { id: "m_send_media_group", name: "send_media_group", category: "📨 Messaging",
        description: "Send 2-10 items as an album",
        signature: "bot.send_media_group(chat_id, media_vec, params)" },
    // Editing
    Method { id: "m_edit_message_text", name: "edit_message_text", category: "✏️ Editing",
        description: "Edit the text of a message",
        signature: "bot.edit_message_text(text, params)" },
    Method { id: "m_edit_message_caption", name: "edit_message_caption", category: "✏️ Editing",
        description: "Edit the caption of a media message",
        signature: "bot.edit_message_caption(params)" },
    Method { id: "m_edit_message_reply_markup", name: "edit_message_reply_markup", category: "✏️ Editing",
        description: "Edit inline keyboard of a message",
        signature: "bot.edit_message_reply_markup(params)" },
    Method { id: "m_delete_message", name: "delete_message", category: "✏️ Editing",
        description: "Delete a message",
        signature: "bot.delete_message(chat_id, message_id)" },
    Method { id: "m_pin_chat_message", name: "pin_chat_message", category: "✏️ Editing",
        description: "Pin a message in a chat",
        signature: "bot.pin_chat_message(chat_id, message_id, params)" },
    // Chat Management
    Method { id: "m_ban_chat_member", name: "ban_chat_member", category: "👥 Chat Mgmt",
        description: "Ban a user from a chat",
        signature: "bot.ban_chat_member(chat_id, user_id, params)" },
    Method { id: "m_unban_chat_member", name: "unban_chat_member", category: "👥 Chat Mgmt",
        description: "Unban a user",
        signature: "bot.unban_chat_member(chat_id, user_id, params)" },
    Method { id: "m_restrict_chat_member", name: "restrict_chat_member", category: "👥 Chat Mgmt",
        description: "Restrict user permissions (mute etc)",
        signature: "bot.restrict_chat_member(chat_id, user_id, permissions, params)" },
    Method { id: "m_promote_chat_member", name: "promote_chat_member", category: "👥 Chat Mgmt",
        description: "Promote or demote a user",
        signature: "bot.promote_chat_member(chat_id, user_id, params)" },
    Method { id: "m_get_chat_member", name: "get_chat_member", category: "👥 Chat Mgmt",
        description: "Get info about a chat member",
        signature: "bot.get_chat_member(chat_id, user_id)" },
    Method { id: "m_get_chat_administrators", name: "get_chat_administrators", category: "👥 Chat Mgmt",
        description: "Get list of chat administrators",
        signature: "bot.get_chat_administrators(chat_id)" },
    Method { id: "m_get_chat_member_count", name: "get_chat_member_count", category: "👥 Chat Mgmt",
        description: "Get member count of a chat",
        signature: "bot.get_chat_member_count(chat_id)" },
    Method { id: "m_set_chat_permissions", name: "set_chat_permissions", category: "👥 Chat Mgmt",
        description: "Set default chat permissions",
        signature: "bot.set_chat_permissions(chat_id, permissions, params)" },
    Method { id: "m_export_chat_invite_link", name: "export_chat_invite_link", category: "👥 Chat Mgmt",
        description: "Generate chat invite link",
        signature: "bot.export_chat_invite_link(chat_id)" },
    // Bot info
    Method { id: "m_get_me", name: "get_me", category: "🤖 Bot Info",
        description: "Get bot information",
        signature: "bot.get_me()" },
    Method { id: "m_get_webhook_info", name: "get_webhook_info", category: "🤖 Bot Info",
        description: "Get current webhook status",
        signature: "bot.get_webhook_info()" },
    Method { id: "m_set_my_commands", name: "set_my_commands", category: "🤖 Bot Info",
        description: "Register bot command list",
        signature: "bot.set_my_commands(commands, params)" },
    Method { id: "m_get_my_commands", name: "get_my_commands", category: "🤖 Bot Info",
        description: "Get registered commands",
        signature: "bot.get_my_commands(params)" },
    // Inline
    Method { id: "m_answer_inline_query", name: "answer_inline_query", category: "🔍 Inline",
        description: "Answer an inline query",
        signature: "bot.answer_inline_query(inline_query_id, results, params)" },
    Method { id: "m_answer_callback_query", name: "answer_callback_query", category: "🔍 Inline",
        description: "Answer a callback query (toast/alert)",
        signature: "bot.answer_callback_query(callback_query_id, params)" },
];

fn method_to_article(m: &Method) -> InlineQueryResult {
    let text = format!(
        "🦀 <b>{}</b>\n{}\n\n\
        <b>Signature:</b>\n<code>{}</code>\n\n\
        <i>via @RustaceBot • tgbotrs v0.1.4</i>",
        m.name, m.description, m.signature
    );
    article(m.id, &format!("{} {}", m.category, m.name), m.description, &text)
}

// ── Rust facts ────────────────────────────────────────────────────────────────

const RUST_FACTS: &[(&str, &str, &str, &str)] = &[
    ("fact_loved", "🏆 Most Loved Language", "9 years in a row on Stack Overflow",
     "🦀 <b>Most Loved Language</b>\n\nRust was voted the most loved programming language on Stack Overflow for <b>9 consecutive years</b> (2016–2024)!\n\n<i>via @RustaceBot • tgbotrs v0.1.4</i>"),
    ("fact_memory", "🔒 Zero Memory Bugs", "Borrow checker catches errors at compile time",
     "🔒 <b>Zero Memory Bugs</b>\n\nRust's borrow checker eliminates null pointer dereferences, dangling pointers, and data races — all at <b>compile time</b>!\n\n<i>via @RustaceBot • tgbotrs v0.1.4</i>"),
    ("fact_linux", "🐧 Rust in Linux Kernel", "Linux 6.1 added official Rust support",
     "🐧 <b>Rust in the Linux Kernel</b>\n\nLinux v6.1 (2022) officially added Rust support — the <b>first new language in the kernel in 30+ years</b>!\n\n<i>via @RustaceBot • tgbotrs v0.1.4</i>"),
    ("fact_perf", "⚡ Zero-Cost Abstractions", "High-level code compiles to the same assembly as C",
     "⚡ <b>Zero-Cost Abstractions</b>\n\nRust's high-level abstractions compile to the <b>same machine code</b> as hand-written C!\n\n<i>via @RustaceBot • tgbotrs v0.1.4</i>"),
    ("fact_bigtech", "🏢 Big Tech Loves Rust", "Microsoft, Google, Meta, Amazon all use Rust",
     "🏢 <b>Big Tech Loves Rust</b>\n\n<b>Microsoft, Google, Meta, Cloudflare, Amazon, Discord</b> all use Rust in production!\n\n<i>via @RustaceBot • tgbotrs v0.1.4</i>"),
    ("fact_tgbotrs", "🤖 About tgbotrs", "165 methods, 285 types, Telegram Bot API 9.4",
     "🤖 <b>tgbotrs v0.1.4</b>\n\nFully-featured, auto-generated Telegram Bot API library for Rust.\n\n✅ All <b>165 methods</b>\n✅ All <b>285 types</b>\n✅ Long-polling &amp; webhook\n\n<a href=\"https://github.com/ankit-chaubey/tgbotrs\">GitHub</a>\n\n<i>via @RustaceBot</i>"),
];

const JOKES: &[(&str, &str, &str, &str)] = &[
    ("joke_borrow", "😂 Borrow Checker Joke", "Why do Rust devs never fight?",
     "😂 <b>Rust Joke</b>\n\nWhy do Rust developers never get into fights?\n\nBecause their <b>borrow checker</b> prevents them from taking things that aren't theirs! 🦀\n\n<i>via @RustaceBot</i>"),
    ("joke_lifetime", "😂 Lifetime Joke", "How many Rust programmers to change a lightbulb?",
     "😂 <b>Rust Joke</b>\n\nHow many Rust programmers does it take to change a light bulb?\n\nNone — the compiler told them the bulb's <b>lifetime</b> is still valid! 💡\n\n<i>via @RustaceBot</i>"),
    ("joke_move", "😂 Move Semantics", "Interviewer: Do you know C++?",
     "😂 <b>Rust Joke</b>\n\nInterviewer: <i>'Do you know C++?'</i>\n\nRust dev: <i>'I used to, but then I <b>moved</b>.'</i> 😎\n\n<i>via @RustaceBot</i>"),
];

// ── Main handler ──────────────────────────────────────────────────────────────

pub async fn handle_inline_query(bot: &Bot, query_id: String, query: &str) {
    let q = query.trim().to_lowercase();

    let results: Vec<InlineQueryResult> = if q.is_empty() {
        // Default: show mix of methods + facts
        let mut r: Vec<InlineQueryResult> = METHODS.iter().take(5).map(method_to_article).collect();
        r.extend(RUST_FACTS.iter().take(3).map(|(id, title, desc, text)| article(id, title, desc, text)));
        r
    } else if q.starts_with("method:") || q.starts_with("fn ") || q.starts_with("bot.") {
        // Explicit method search
        let search = q.trim_start_matches("method:").trim_start_matches("fn ").trim_start_matches("bot.").trim();
        METHODS.iter()
            .filter(|m| m.name.contains(search) || m.description.to_lowercase().contains(search) || m.category.to_lowercase().contains(search))
            .take(10)
            .map(method_to_article)
            .collect()
    } else if q.contains("send") || q.contains("message") || q.contains("photo") || q.contains("video") || q.contains("media") {
        METHODS.iter()
            .filter(|m| m.name.contains(&q) || m.description.to_lowercase().contains(&q) || m.category.to_lowercase().contains(&q))
            .take(10)
            .map(method_to_article)
            .collect()
    } else if q.contains("ban") || q.contains("mute") || q.contains("kick") || q.contains("restrict") || q.contains("admin") || q.contains("promote") || q.contains("member") {
        METHODS.iter()
            .filter(|m| m.name.contains(&q) || m.description.to_lowercase().contains(&q) || m.category.contains("Chat"))
            .take(10)
            .map(method_to_article)
            .collect()
    } else if q.contains("joke") || q.contains("fun") || q.contains("lol") {
        JOKES.iter().map(|(id, title, desc, text)| article(id, title, desc, text)).collect()
    } else if q.contains("fact") || q.contains("rust") || q.contains("memory") || q.contains("linux") {
        RUST_FACTS.iter().map(|(id, title, desc, text)| article(id, title, desc, text)).collect()
    } else if q.contains("about") || q.contains("bot") || q.contains("tgbotrs") || q.contains("lib") {
        vec![
            article("about_bot", "🤖 About Rustace Bot", "Official showcase for tgbotrs",
                "🦀 <b>Rustace Bot</b> — @RustaceBot\n\nOfficial showcase for <a href=\"https://github.com/ankit-chaubey/tgbotrs\">tgbotrs</a> — Rust Telegram Bot API library.\n✅ 165 methods | 285 types | Async | Polling | Webhook"),
            article("about_install", "📦 Install tgbotrs", "tgbotrs = \"0.1.4\"",
                "📦 <b>Install tgbotrs</b>\n\n<pre>[dependencies]\ntgbotrs = \"0.1.4\"\ntokio   = { version = \"1\", features = [\"full\"] }</pre>\n\n<a href=\"https://crates.io/crates/tgbotrs\">crates.io</a> | <a href=\"https://docs.rs/tgbotrs\">docs.rs</a>"),
        ]
    } else {
        // General search across methods + facts
        let mut results: Vec<InlineQueryResult> = METHODS.iter()
            .filter(|m| m.name.contains(&q) || m.description.to_lowercase().contains(&q) || m.category.to_lowercase().contains(&q))
            .take(5)
            .map(method_to_article)
            .collect();
        let fact_results: Vec<InlineQueryResult> = RUST_FACTS.iter()
            .filter(|(_, title, desc, _)| title.to_lowercase().contains(&q) || desc.to_lowercase().contains(&q))
            .take(3)
            .map(|(id, title, desc, text)| article(id, title, desc, text))
            .collect();
        results.extend(fact_results);
        if results.is_empty() {
            // Fallback: top methods
            METHODS.iter().take(5).map(method_to_article).collect()
        } else {
            results
        }
    };

    let params = AnswerInlineQueryParams::new()
        .cache_time(10_i64)
        .button(Box::new(InlineQueryResultsButton {
            text: "🦀 Open Rustace Bot".into(),
            web_app: None,
            start_parameter: Some("inline".into()),
        }));

    let _ = bot.answer_inline_query(query_id, results, Some(params)).await;
}
