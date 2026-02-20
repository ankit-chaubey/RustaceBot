// ════════════════════════════════════════════════════════════════
//  Rustace Bot — Broadcast & Media Handlers
//
//  /send  <text + optional button lines>    — custom message + buttons
//  /post  <text + optional button lines>    — styled broadcast frame
//  /img   <url> [caption + buttons]         — send photo from URL
//  /vid   <url> [caption + buttons]         — send video from URL
//  /aud   <url> [caption + buttons]         — send audio from URL
//  /doc   <url> [caption + buttons]         — send document from URL
//  /buttons                                 — colourful button showcase
//  /sendhelp                                — guide for /send /post syntax
//
//  ── Button syntax ──────────────────────────────────────────────
//  Add button lines AFTER your text, one line per row:
//    [Label | callback_data]
//    [Label | https://url] [Label2 | data2]   ← same row, side by side
//  ───────────────────────────────────────────────────────────────
// ════════════════════════════════════════════════════════════════

use tgbotrs::{
    gen_methods::{
        SendAudioParams, SendDocumentParams, SendMessageParams,
        SendPhotoParams, SendVideoParams,
    },
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    Bot, ReplyMarkup,
};
use super::commands::btn;

// ── Button-line parser ────────────────────────────────────────────────────────
// Parses: [Label | value]  [Label2 | value2]  on one line → one row

fn parse_button_rows(lines: &[&str]) -> Vec<Vec<InlineKeyboardButton>> {
    lines.iter().filter_map(|line| {
        let line = line.trim();
        if !line.contains('[') || !line.contains('|') || !line.contains(']') { return None; }
        let mut row = vec![];
        let mut rest = line;
        while let Some(op) = rest.find('[') {
            let after = &rest[op+1..];
            if let Some(cl) = after.find(']') {
                let inner = &after[..cl];
                if let Some(pi) = inner.find('|') {
                    let label = inner[..pi].trim().to_string();
                    let value = inner[pi+1..].trim().to_string();
                    if !label.is_empty() && !value.is_empty() {
                        let ib = if value.starts_with("http://") || value.starts_with("https://") || value.starts_with("tg://") {
                            InlineKeyboardButton { text: label, url: Some(value), ..Default::default() }
                        } else {
                            InlineKeyboardButton { text: label, callback_data: Some(value), ..Default::default() }
                        };
                        row.push(ib);
                    }
                }
                rest = &after[cl+1..];
            } else { break; }
        }
        if row.is_empty() { None } else { Some(row) }
    }).collect()
}

// Split raw text into (message_body, button_rows)
fn split_body_buttons(raw: &str) -> (String, Vec<Vec<InlineKeyboardButton>>) {
    let lines: Vec<&str> = raw.lines().collect();
    let (text_lines, btn_lines): (Vec<&str>, Vec<&str>) = lines.iter().partition(|l| {
        let t = l.trim();
        !(t.contains('[') && t.contains('|') && t.contains(']'))
    });
    let body = text_lines.join("\n").trim().to_string();
    let rows = parse_button_rows(&btn_lines);
    (body, rows)
}

async fn err(bot: &Bot, chat_id: i64, text: &str) {
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Menu","main_menu")]] };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

// ── /send ─────────────────────────────────────────────────────────────────────

pub async fn handle_send(bot: &Bot, chat_id: i64, raw: &str) {
    if raw.trim().is_empty() {
        err(bot, chat_id,
            "⚠️ <b>Usage:</b>\n\
            <pre>/send Your message here\n\
            [✅ Button | callback_data] [🔗 Link | https://url]\n\
            [Another Row | data2]</pre>\n\n\
            Buttons on the same line → same row.\n\
            Different lines → different rows.").await;
        return;
    }
    let (body, rows) = split_body_buttons(raw);
    if body.is_empty() { err(bot, chat_id, "⚠️ Please add message text above the button lines.").await; return; }
    let kb = InlineKeyboardMarkup { inline_keyboard: rows };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, body, Some(p)).await;
}

// ── /post ─────────────────────────────────────────────────────────────────────

pub async fn handle_post(bot: &Bot, chat_id: i64, raw: &str) {
    if raw.trim().is_empty() {
        err(bot, chat_id,
            "⚠️ <b>Usage:</b>\n\
            <pre>/post 📢 Big announcement text!\n\
            More details here...\n\
            [✅ Accept | ok] [❌ Decline | no]\n\
            [🌐 Website | https://example.com]</pre>").await;
        return;
    }
    let (body, rows) = split_body_buttons(raw);
    if body.is_empty() { err(bot, chat_id, "⚠️ Please add post text above the button lines.").await; return; }
    let styled = format!("━━━━━━━━━━━━━━━━━━━━\n{}\n━━━━━━━━━━━━━━━━━━━━", body);
    let kb = InlineKeyboardMarkup { inline_keyboard: rows };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, styled, Some(p)).await;
}

// ── /img <url> [caption + buttons] ───────────────────────────────────────────

pub async fn handle_img(bot: &Bot, chat_id: i64, args: &[&str]) {
    if args.is_empty() {
        err(bot, chat_id,
            "⚠️ <b>Usage:</b>\n\
            <code>/img https://url.to/image.jpg Caption here</code>\n\n\
            With buttons:\n\
            <pre>/img https://i.imgur.com/abc.jpg 🌅 Sunset!\n[❤️ Like | liked] [🔗 Source | https://imgur.com]</pre>").await;
        return;
    }
    let url  = args[0];
    let rest = args[1..].join(" ");
    let (caption, rows) = split_body_buttons(&rest);
    let mut p = SendPhotoParams::new().parse_mode("HTML");
    if !caption.is_empty() { p = p.caption(caption); }
    if !rows.is_empty() { p = p.reply_markup(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup { inline_keyboard: rows })); }
    if let Err(e) = bot.send_photo(chat_id, url, Some(p)).await {
        err(bot, chat_id, &format!("❌ <b>Photo send failed:</b> <code>{}</code>\n<i>Use a direct image URL (jpg/png/webp/gif).</i>", e)).await;
    }
}

// ── /vid <url> [caption + buttons] ───────────────────────────────────────────

pub async fn handle_vid(bot: &Bot, chat_id: i64, args: &[&str]) {
    if args.is_empty() {
        err(bot, chat_id, "⚠️ <b>Usage:</b> <code>/vid https://url.to/video.mp4 [caption]</code>").await;
        return;
    }
    let url  = args[0];
    let rest = args[1..].join(" ");
    let (caption, rows) = split_body_buttons(&rest);
    let mut p = SendVideoParams::new().parse_mode("HTML");
    if !caption.is_empty() { p = p.caption(caption); }
    if !rows.is_empty() { p = p.reply_markup(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup { inline_keyboard: rows })); }
    if let Err(e) = bot.send_video(chat_id, url, Some(p)).await {
        err(bot, chat_id, &format!("❌ <b>Video send failed:</b> <code>{}</code>", e)).await;
    }
}

// ── /aud <url> [caption + buttons] ───────────────────────────────────────────

pub async fn handle_aud(bot: &Bot, chat_id: i64, args: &[&str]) {
    if args.is_empty() {
        err(bot, chat_id, "⚠️ <b>Usage:</b> <code>/aud https://url.to/audio.mp3 [caption]</code>").await;
        return;
    }
    let url  = args[0];
    let rest = args[1..].join(" ");
    let (caption, rows) = split_body_buttons(&rest);
    let mut p = SendAudioParams::new().parse_mode("HTML");
    if !caption.is_empty() { p = p.caption(caption); }
    if !rows.is_empty() { p = p.reply_markup(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup { inline_keyboard: rows })); }
    if let Err(e) = bot.send_audio(chat_id, url, Some(p)).await {
        err(bot, chat_id, &format!("❌ <b>Audio send failed:</b> <code>{}</code>", e)).await;
    }
}

// ── /doc <url> [caption + buttons] ───────────────────────────────────────────

pub async fn handle_doc(bot: &Bot, chat_id: i64, args: &[&str]) {
    if args.is_empty() {
        err(bot, chat_id, "⚠️ <b>Usage:</b> <code>/doc https://url.to/file.pdf [caption]</code>").await;
        return;
    }
    let url  = args[0];
    let rest = args[1..].join(" ");
    let (caption, rows) = split_body_buttons(&rest);
    let mut p = SendDocumentParams::new().parse_mode("HTML");
    if !caption.is_empty() { p = p.caption(caption); }
    if !rows.is_empty() { p = p.reply_markup(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup { inline_keyboard: rows })); }
    if let Err(e) = bot.send_document(chat_id, url, Some(p)).await {
        err(bot, chat_id, &format!("❌ <b>Document send failed:</b> <code>{}</code>", e)).await;
    }
}

// ── /buttons — colourful button showcase ─────────────────────────────────────

pub async fn handle_buttons_showcase(bot: &Bot, chat_id: i64) {
    let text = "🎨 <b>Inline Button Showcase</b>\n\n\
        All powered by tgbotrs <code>InlineKeyboardMarkup</code>!\n\
        Buttons can have callbacks, URLs, or alert/toast responses. Try them! 👇";

    fn ib(label: &str, cb: &str) -> InlineKeyboardButton {
        InlineKeyboardButton { text: label.into(), callback_data: Some(cb.into()), ..Default::default() }
    }
    fn url_btn(label: &str, url: &str) -> InlineKeyboardButton {
        InlineKeyboardButton { text: label.into(), url: Some(url.into()), ..Default::default() }
    }

    let kb = InlineKeyboardMarkup { inline_keyboard: vec![
        vec![ib("🔴 Red", "btn_color"), ib("🟡 Yellow", "btn_color"), ib("🟢 Green", "btn_color")],
        vec![ib("🔵 Blue", "btn_color"), ib("🟣 Purple", "btn_color"), ib("🟠 Orange", "btn_color")],
        vec![ib("⭐ Star", "btn_shape"), ib("💎 Diamond", "btn_shape"), ib("🎯 Target", "btn_shape")],
        vec![ib("🚨 Alert Popup",   "alert_demo"), ib("📢 Toast Notif", "toast_demo")],
        vec![ib("🔔 Callback URL", "cb_url_demo"), ib("💬 Silent Toast", "notif_demo")],
        vec![url_btn("📦 crates.io", "https://crates.io/crates/tgbotrs"),
             url_btn("📖 docs.rs",   "https://docs.rs/tgbotrs")],
        vec![url_btn("🐙 GitHub Library", "https://github.com/ankit-chaubey/tgbotrs")],
        vec![ib("⬅️ Main Menu", "main_menu")],
    ]};
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

// ── /sendhelp ─────────────────────────────────────────────────────────────────

pub async fn handle_send_help(bot: &Bot, chat_id: i64) {
    let text = "📡 <b>Send &amp; Post Guide</b>\n\n\
        <b>Commands:</b>\n\
        /send — Custom message with buttons\n\
        /post — Styled broadcast frame + buttons\n\
        /img  — Photo from URL + buttons\n\
        /vid  — Video from URL + buttons\n\
        /aud  — Audio from URL + buttons\n\
        /doc  — Document from URL + buttons\n\n\
        <b>📝 Simple message:</b>\n\
        <pre>/send 🎉 Hello everyone!</pre>\n\n\
        <b>🔘 Callback buttons:</b>\n\
        <pre>/send Choose:\n[✅ Option A | opt_a] [❌ Option B | opt_b]</pre>\n\n\
        <b>🔗 URL buttons:</b>\n\
        <pre>/send Visit us:\n[GitHub | https://github.com] [Docs | https://docs.rs]</pre>\n\n\
        <b>📸 Photo with caption + buttons:</b>\n\
        <pre>/img https://i.imgur.com/x.jpg 🌅 Sunset\n[❤️ Like | liked] [🔗 Src | https://imgur.com]</pre>\n\n\
        <b>📢 Broadcast post:</b>\n\
        <pre>/post 📢 Big news!\nWe just launched v2.0\n[🔗 Read more | https://example.com]</pre>\n\n\
        <b>Button rules:</b>\n\
        • <code>[Label | callback_data]</code> → inline button\n\
        • <code>[Label | https://url]</code> → URL button\n\
        • Same line = same row · Different lines = different rows";

    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Menu", "main_menu")]] };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}
