// ════════════════════════════════════════════════════════════════
//  Rustace Bot — Inline Query Handler
// ════════════════════════════════════════════════════════════════

use rand::Rng;
use tgbotrs::{
    gen_methods::AnswerInlineQueryParams,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResult,
        InlineQueryResultArticle, InlineQueryResultsButton, InputMessageContent,
        InputTextMessageContent,
    },
    Bot,
};

fn article(id: &str, title: &str, description: &str, text: &str) -> InlineQueryResult {
    InlineQueryResult::Article(InlineQueryResultArticle {
        r#type: "article".to_string(),
        id: id.to_string(),
        title: title.to_string(),
        input_message_content: InputMessageContent::Text(InputTextMessageContent {
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

pub async fn handle_inline_query(bot: &Bot, query_id: String, query: &str) {
    let q = query.trim().to_lowercase();

    let results: Vec<InlineQueryResult> = if q.is_empty() || q.contains("fact") || q.contains("rust") {
        RUST_FACTS.iter().map(|(id, title, desc, text)| article(id, title, desc, text)).collect()
    } else if q.contains("joke") || q.contains("fun") {
        JOKES.iter().map(|(id, title, desc, text)| article(id, title, desc, text)).collect()
    } else if q.contains("about") || q.contains("bot") || q.contains("lib") {
        vec![
            article("about_bot", "🤖 About Rustace Bot", "Official showcase for tgbotrs",
                "🦀 <b>Rustace Bot</b> — @RustaceBot\n\nOfficial showcase for <a href=\"https://github.com/ankit-chaubey/tgbotrs\">tgbotrs</a> — Rust Telegram Bot API library.\n✅ 165 methods | 285 types | Async | Polling | Webhook"),
            article("about_install", "📦 Install tgbotrs", "tgbotrs = \"0.1.4\"",
                "📦 <b>Install tgbotrs</b>\n\n<pre>[dependencies]\ntgbotrs = \"0.1.4\"\ntokio   = { version = \"1\", features = [\"full\"] }</pre>\n\n<a href=\"https://crates.io/crates/tgbotrs\">crates.io</a> | <a href=\"https://docs.rs/tgbotrs\">docs.rs</a>"),
        ]
    } else {
        let mut results: Vec<InlineQueryResult> = RUST_FACTS.iter().take(4)
            .map(|(id, title, desc, text)| article(id, title, desc, text))
            .collect();
        results.push(article("about_default", "🤖 About @RustaceBot", "Powered by tgbotrs",
            "🦀 <b>@RustaceBot</b> — Powered by tgbotrs v0.1.4\n\nType <code>fact</code>, <code>joke</code>, or <code>about</code> to explore!"));
        results
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
