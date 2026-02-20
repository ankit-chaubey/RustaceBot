// ════════════════════════════════════════════════════════════════
//  Rustace Bot — Notes System
//  /note <name> <content>   save a note
//  /get <name>              get a note   (also: #name in chat)
//  /notes                   list all notes
//  /delnote <name>          delete a note
// ════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tgbotrs::{gen_methods::SendMessageParams, types::InlineKeyboardMarkup, Bot, ReplyMarkup};
use super::commands::btn;

pub type NoteStore = Arc<Mutex<HashMap<(i64, String), String>>>;

pub fn new_note_store() -> NoteStore {
    Arc::new(Mutex::new(HashMap::new()))
}

fn he(s: &str) -> String { s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;") }

async fn reply(bot: &Bot, chat_id: i64, text: &str) {
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Menu","main_menu")]] };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

// /note name content
pub async fn handle_save_note(bot: &Bot, chat_id: i64, args: &[&str], store: &NoteStore) {
    if args.len() < 2 {
        reply(bot, chat_id,
            "⚠️ <b>Usage:</b> <code>/note name content</code>\n\n\
            <b>Examples:</b>\n\
            <code>/note rules 📜 No spam, no links, be kind!</code>\n\
            <code>/note welcome 🎉 Welcome to our group!</code>\n\n\
            <i>Retrieve with <code>/get name</code> or just type <code>#name</code></i>").await;
        return;
    }
    let name    = args[0].to_lowercase();
    let content = args[1..].join(" ");
    { store.lock().unwrap().insert((chat_id, name.clone()), content.clone()); }
    reply(bot, chat_id, &format!(
        "📝 <b>Note saved!</b>\n\n\
        📌 Name: <code>{name}</code>\n\
        📄 Content: {content}\n\n\
        <i>Get it: <code>/get {name}</code> or <code>#{name}</code></i>",
        name=he(&name), content=he(&content)
    )).await;
}

// /get name  or  #name trigger
pub async fn handle_get_note(bot: &Bot, chat_id: i64, arg: Option<&str>, store: &NoteStore) {
    let Some(raw) = arg else {
        reply(bot, chat_id, "⚠️ <b>Usage:</b> <code>/get note_name</code>").await;
        return;
    };
    let name = raw.to_lowercase().trim_start_matches('#').to_string();
    let content = { store.lock().unwrap().get(&(chat_id, name.clone())).cloned() };
    match content {
        Some(c) => { let _ = bot.send_message(chat_id, c, Some(SendMessageParams::new().parse_mode("HTML"))).await; }
        None => reply(bot, chat_id, &format!(
            "❓ <b>Note not found:</b> <code>{}</code>\nUse <code>/notes</code> to see all notes.",
            he(&name))).await,
    }
}

// /notes
pub async fn handle_list_notes(bot: &Bot, chat_id: i64, store: &NoteStore) {
    let mut names: Vec<String> = {
        let s = store.lock().unwrap();
        s.keys().filter(|(cid,_)| *cid == chat_id).map(|(_,n)| n.clone()).collect()
    };
    if names.is_empty() {
        reply(bot, chat_id, "📂 <b>No notes saved.</b>\nUse <code>/note name content</code> to save one.").await;
        return;
    }
    names.sort();
    let list: String = names.iter().map(|n| format!("📌 <code>#{}</code>\n", he(n))).collect();
    reply(bot, chat_id, &format!(
        "📋 <b>Saved Notes</b> ({} total)\n\n{}\n<i>Get any note: <code>/get name</code> or <code>#name</code></i>",
        names.len(), list
    )).await;
}

// /delnote name
pub async fn handle_del_note(bot: &Bot, chat_id: i64, arg: Option<&str>, store: &NoteStore) {
    let Some(n) = arg else {
        reply(bot, chat_id, "⚠️ <b>Usage:</b> <code>/delnote name</code>").await;
        return;
    };
    let name = n.to_lowercase();
    let removed = { store.lock().unwrap().remove(&(chat_id, name.clone())).is_some() };
    if removed {
        reply(bot, chat_id, &format!("🗑️ <b>Note deleted:</b> <code>{}</code>", he(&name))).await;
    } else {
        reply(bot, chat_id, &format!("❓ No note named: <code>{}</code>", he(&name))).await;
    }
}

// #hashtag auto-trigger — returns true if handled
pub async fn check_hashtag_note(bot: &Bot, chat_id: i64, text: &str, store: &NoteStore) -> bool {
    if !text.starts_with('#') { return false; }
    let name = text.split_whitespace().next().unwrap_or("")
        .trim_start_matches('#').to_lowercase();
    if name.is_empty() { return false; }
    let content = { store.lock().unwrap().get(&(chat_id, name)).cloned() };
    if let Some(c) = content {
        let _ = bot.send_message(chat_id, c, Some(SendMessageParams::new().parse_mode("HTML"))).await;
        true
    } else { false }
}
