// ════════════════════════════════════════════════════════════════
//  Rustace Bot — Filters System
//  /filter <keyword> <response>   save a keyword auto-reply
//  /delfilter <keyword>           delete a filter
//  /filters                       list all active filters
//  Auto-triggers when any message contains a keyword.
// ════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tgbotrs::{gen_methods::SendMessageParams, types::InlineKeyboardMarkup, Bot, ReplyMarkup};
use super::commands::btn;

pub type FilterStore = Arc<Mutex<HashMap<(i64, String), String>>>;

pub fn new_filter_store() -> FilterStore {
    Arc::new(Mutex::new(HashMap::new()))
}

fn he(s: &str) -> String { s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;") }

async fn reply(bot: &Bot, chat_id: i64, text: &str) {
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Menu","main_menu")]] };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

// /filter keyword response text
pub async fn handle_set_filter(bot: &Bot, chat_id: i64, args: &[&str], store: &FilterStore) {
    if args.len() < 2 {
        reply(bot, chat_id,
            "⚠️ <b>Usage:</b> <code>/filter keyword response text</code>\n\n\
            <b>Examples:</b>\n\
            <code>/filter hello 👋 Hello there!</code>\n\
            <code>/filter rules 📜 Please read the rules!</code>\n\
            <code>/filter links 🚫 No links allowed.</code>\n\n\
            <i>The bot will auto-reply whenever anyone says the keyword.</i>").await;
        return;
    }
    let keyword  = args[0].to_lowercase();
    let response = args[1..].join(" ");
    { store.lock().unwrap().insert((chat_id, keyword.clone()), response.clone()); }
    reply(bot, chat_id, &format!(
        "✅ <b>Filter saved!</b>\n\n🔑 Keyword: <code>{}</code>\n💬 Response: {}",
        he(&keyword), he(&response)
    )).await;
}

// /delfilter keyword
pub async fn handle_del_filter(bot: &Bot, chat_id: i64, arg: Option<&str>, store: &FilterStore) {
    let Some(kw) = arg else {
        reply(bot, chat_id, "⚠️ <b>Usage:</b> <code>/delfilter keyword</code>").await;
        return;
    };
    let keyword = kw.to_lowercase();
    let removed = { store.lock().unwrap().remove(&(chat_id, keyword.clone())).is_some() };
    if removed {
        reply(bot, chat_id, &format!("🗑️ <b>Filter deleted:</b> <code>{}</code>", he(&keyword))).await;
    } else {
        reply(bot, chat_id, &format!("❓ No filter found for: <code>{}</code>", he(&keyword))).await;
    }
}

// /filters  — list all
pub async fn handle_list_filters(bot: &Bot, chat_id: i64, store: &FilterStore) {
    let mut pairs: Vec<(String, String)> = {
        let s = store.lock().unwrap();
        s.iter().filter(|((cid,_),_)| *cid == chat_id)
            .map(|((_,k),v)| (k.clone(), v.clone())).collect()
    };
    if pairs.is_empty() {
        reply(bot, chat_id, "📂 <b>No filters set.</b>\nUse <code>/filter keyword response</code> to add one.").await;
        return;
    }
    pairs.sort_by(|a,b| a.0.cmp(&b.0));
    let list: String = pairs.iter().map(|(k,v)| {
        let preview = if v.len() > 35 { format!("{}…", &v[..35]) } else { v.clone() };
        format!("🔑 <code>{}</code> → {}\n", he(k), he(&preview))
    }).collect();
    reply(bot, chat_id, &format!("📋 <b>Active Filters</b> ({} total)\n\n{}", pairs.len(), list)).await;
}

// Auto-trigger: called on every message. Returns true if matched.
pub async fn check_filters(bot: &Bot, chat_id: i64, text: &str, store: &FilterStore) -> bool {
    let lower = text.to_lowercase();
    let matched: Option<String> = {
        let s = store.lock().unwrap();
        s.iter()
            .filter(|((cid, kw), _)| *cid == chat_id && lower.contains(kw.as_str()))
            .map(|(_, v)| v.clone())
            .next()
    };
    if let Some(resp) = matched {
        let p = SendMessageParams::new().parse_mode("HTML");
        let _ = bot.send_message(chat_id, resp, Some(p)).await;
        true
    } else { false }
}
