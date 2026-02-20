// ════════════════════════════════════════════════════════════════
//  Rustace Bot — Moderation Handlers
//  Commands: ban, unban, kick, mute, unmute, warn, unwarn, delete,
//            pin, unpin, ro, unro
//  All commands work by replying to the target user's message.
// ════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tgbotrs::{
    gen_methods::{
        BanChatMemberParams, RestrictChatMemberParams, SendMessageParams,
        UnbanChatMemberParams,
    },
    types::{ChatPermissions, InlineKeyboardMarkup},
    Bot, ChatId, ReplyMarkup,
};

use super::commands::btn;

// ── Warn store (in-memory, per chat:user) ────────────────────────────────────
//   Resets on bot restart — good enough for most bots.

pub type WarnStore = Arc<Mutex<HashMap<(i64, i64), u8>>>;

pub fn new_warn_store() -> WarnStore {
    Arc::new(Mutex::new(HashMap::new()))
}

// ── Permission helpers ───────────────────────────────────────────────────────

fn no_perms() -> ChatPermissions {
    ChatPermissions {
        can_send_messages: Some(false),
        can_send_audios: Some(false),
        can_send_documents: Some(false),
        can_send_photos: Some(false),
        can_send_videos: Some(false),
        can_send_video_notes: Some(false),
        can_send_voice_notes: Some(false),
        can_send_polls: Some(false),
        can_send_other_messages: Some(false),
        can_add_web_page_previews: Some(false),
        can_change_info: Some(false),
        can_invite_users: Some(false),
        can_pin_messages: Some(false),
        can_manage_topics: Some(false),
    }
}

fn all_perms() -> ChatPermissions {
    ChatPermissions {
        can_send_messages: Some(true),
        can_send_audios: Some(true),
        can_send_documents: Some(true),
        can_send_photos: Some(true),
        can_send_videos: Some(true),
        can_send_video_notes: Some(true),
        can_send_voice_notes: Some(true),
        can_send_polls: Some(true),
        can_send_other_messages: Some(true),
        can_add_web_page_previews: Some(true),
        can_change_info: Some(false),
        can_invite_users: Some(true),
        can_pin_messages: Some(false),
        can_manage_topics: Some(false),
    }
}

// ── Reply helper: sends a moderation result message ──────────────────────────

async fn mod_msg(bot: &Bot, chat_id: i64, text: &str, back_cb: &str) {
    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn(back_cb, back_cb)]],
    };
    let p = SendMessageParams::new()
        .parse_mode("HTML")
        .reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

// ── Parse duration arg like "1h", "30m", "7d" → unix timestamp ──────────────

fn parse_duration_secs(arg: Option<&str>) -> Option<i64> {
    let s = arg?;
    let (num_str, unit) = if s.ends_with('d') {
        (&s[..s.len()-1], 86400i64)
    } else if s.ends_with('h') {
        (&s[..s.len()-1], 3600i64)
    } else if s.ends_with('m') {
        (&s[..s.len()-1], 60i64)
    } else {
        return None;
    };
    let n: i64 = num_str.parse().ok()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    Some(now + n * unit)
}

// ════════════════════════════════════════════════════════════════
//  /ban [duration] — Reply to a message to ban that user
//  Duration examples: /ban 7d  /ban 2h  /ban 30m  (omit = forever)
// ════════════════════════════════════════════════════════════════

pub async fn handle_ban(
    bot: &Bot,
    chat_id: i64,
    reply_user_id: Option<i64>,
    reply_user_name: Option<&str>,
    arg: Option<&str>,
) {
    let (target_id, target_name) = match (reply_user_id, reply_user_name) {
        (Some(id), Some(name)) => (id, name.to_string()),
        _ => {
            mod_msg(bot, chat_id,
                "⚠️ <b>Usage:</b> Reply to a message with <code>/ban</code> or <code>/ban 7d</code>",
                "main_menu").await;
            return;
        }
    };

    let mut params = BanChatMemberParams::new().revoke_messages(true);
    let duration_label;

    if let Some(until) = parse_duration_secs(arg) {
        params = params.until_date(until);
        duration_label = format!("for <b>{}</b>", arg.unwrap_or("?"));
    } else {
        duration_label = "<b>permanently</b>".to_string();
    }

    match bot.ban_chat_member(ChatId::from(chat_id), target_id, Some(params)).await {
        Ok(_) => {
            mod_msg(bot, chat_id,
                &format!("🔨 <b>Banned</b> <a href=\"tg://user?id={}\">{}</a> {}\n\n<i>Messages revoked.</i>",
                    target_id, html_escape(&target_name), duration_label),
                "main_menu").await;
        }
        Err(e) => {
            mod_msg(bot, chat_id,
                &format!("❌ <b>Ban failed:</b> <code>{}</code>\n\n<i>Bot must be admin with ban rights.</i>", e),
                "main_menu").await;
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  /unban — Reply to a message to unban that user
// ════════════════════════════════════════════════════════════════

pub async fn handle_unban(
    bot: &Bot,
    chat_id: i64,
    reply_user_id: Option<i64>,
    reply_user_name: Option<&str>,
) {
    let (target_id, target_name) = match (reply_user_id, reply_user_name) {
        (Some(id), Some(name)) => (id, name.to_string()),
        _ => {
            mod_msg(bot, chat_id,
                "⚠️ <b>Usage:</b> Reply to a message with <code>/unban</code>",
                "main_menu").await;
            return;
        }
    };

    let params = UnbanChatMemberParams::new().only_if_banned(true);
    match bot.unban_chat_member(ChatId::from(chat_id), target_id, Some(params)).await {
        Ok(_) => {
            mod_msg(bot, chat_id,
                &format!("✅ <b>Unbanned</b> <a href=\"tg://user?id={}\">{}</a>\n\n<i>User can now rejoin via invite link.</i>",
                    target_id, html_escape(&target_name)),
                "main_menu").await;
        }
        Err(e) => {
            mod_msg(bot, chat_id,
                &format!("❌ <b>Unban failed:</b> <code>{}</code>", e),
                "main_menu").await;
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  /kick — Ban then immediately unban (removes from group, can rejoin)
// ════════════════════════════════════════════════════════════════

pub async fn handle_kick(
    bot: &Bot,
    chat_id: i64,
    reply_user_id: Option<i64>,
    reply_user_name: Option<&str>,
) {
    let (target_id, target_name) = match (reply_user_id, reply_user_name) {
        (Some(id), Some(name)) => (id, name.to_string()),
        _ => {
            mod_msg(bot, chat_id,
                "⚠️ <b>Usage:</b> Reply to a message with <code>/kick</code>",
                "main_menu").await;
            return;
        }
    };

    let ban_ok = bot.ban_chat_member(ChatId::from(chat_id), target_id, None).await.is_ok();
    if ban_ok {
        let _ = bot.unban_chat_member(ChatId::from(chat_id), target_id, None).await;
        mod_msg(bot, chat_id,
            &format!("👢 <b>Kicked</b> <a href=\"tg://user?id={}\">{}</a>\n\n<i>They were removed but can rejoin via invite link.</i>",
                target_id, html_escape(&target_name)),
            "main_menu").await;
    } else {
        mod_msg(bot, chat_id,
            "❌ <b>Kick failed.</b> Bot must be admin with ban rights.",
            "main_menu").await;
    }
}

// ════════════════════════════════════════════════════════════════
//  /mute [duration] — Restrict all permissions
//  Duration: /mute 1h  /mute 30m  /mute 7d  (omit = forever)
// ════════════════════════════════════════════════════════════════

pub async fn handle_mute(
    bot: &Bot,
    chat_id: i64,
    reply_user_id: Option<i64>,
    reply_user_name: Option<&str>,
    arg: Option<&str>,
) {
    let (target_id, target_name) = match (reply_user_id, reply_user_name) {
        (Some(id), Some(name)) => (id, name.to_string()),
        _ => {
            mod_msg(bot, chat_id,
                "⚠️ <b>Usage:</b> Reply to a message with <code>/mute</code> or <code>/mute 1h</code>",
                "main_menu").await;
            return;
        }
    };

    let mut params = RestrictChatMemberParams::new();
    let duration_label;

    if let Some(until) = parse_duration_secs(arg) {
        params = params.until_date(until);
        duration_label = format!("for <b>{}</b>", arg.unwrap_or("?"));
    } else {
        duration_label = "<b>permanently</b>".to_string();
    }

    match bot.restrict_chat_member(ChatId::from(chat_id), target_id, no_perms(), Some(params)).await {
        Ok(_) => {
            mod_msg(bot, chat_id,
                &format!("🔇 <b>Muted</b> <a href=\"tg://user?id={}\">{}</a> {}\n\n<i>All send permissions removed.</i>",
                    target_id, html_escape(&target_name), duration_label),
                "main_menu").await;
        }
        Err(e) => {
            mod_msg(bot, chat_id,
                &format!("❌ <b>Mute failed:</b> <code>{}</code>\n\n<i>Bot must be admin with restrict rights.</i>", e),
                "main_menu").await;
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  /unmute — Restore all standard permissions
// ════════════════════════════════════════════════════════════════

pub async fn handle_unmute(
    bot: &Bot,
    chat_id: i64,
    reply_user_id: Option<i64>,
    reply_user_name: Option<&str>,
) {
    let (target_id, target_name) = match (reply_user_id, reply_user_name) {
        (Some(id), Some(name)) => (id, name.to_string()),
        _ => {
            mod_msg(bot, chat_id,
                "⚠️ <b>Usage:</b> Reply to a message with <code>/unmute</code>",
                "main_menu").await;
            return;
        }
    };

    match bot.restrict_chat_member(ChatId::from(chat_id), target_id, all_perms(), None).await {
        Ok(_) => {
            mod_msg(bot, chat_id,
                &format!("🔊 <b>Unmuted</b> <a href=\"tg://user?id={}\">{}</a>\n\n<i>Standard permissions restored.</i>",
                    target_id, html_escape(&target_name)),
                "main_menu").await;
        }
        Err(e) => {
            mod_msg(bot, chat_id,
                &format!("❌ <b>Unmute failed:</b> <code>{}</code>", e),
                "main_menu").await;
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  /warn — Warn a user. At 3 warnings → auto-ban.
// ════════════════════════════════════════════════════════════════

pub async fn handle_warn(
    bot: &Bot,
    chat_id: i64,
    reply_user_id: Option<i64>,
    reply_user_name: Option<&str>,
    warn_store: &WarnStore,
) {
    let (target_id, target_name) = match (reply_user_id, reply_user_name) {
        (Some(id), Some(name)) => (id, name.to_string()),
        _ => {
            mod_msg(bot, chat_id,
                "⚠️ <b>Usage:</b> Reply to a message with <code>/warn</code>",
                "main_menu").await;
            return;
        }
    };

    let count = {
        let mut store = warn_store.lock().unwrap();
        let entry = store.entry((chat_id, target_id)).or_insert(0);
        *entry += 1;
        *entry
    };

    if count >= 3 {
        // Auto-ban at 3 warnings
        {
            let mut store = warn_store.lock().unwrap();
            store.remove(&(chat_id, target_id));
        }
        let _ = bot.ban_chat_member(ChatId::from(chat_id), target_id,
            Some(BanChatMemberParams::new().revoke_messages(true))).await;
        mod_msg(bot, chat_id,
            &format!("🔨 <a href=\"tg://user?id={}\">{}</a> reached <b>3/3 warnings</b> and was automatically <b>banned</b>.",
                target_id, html_escape(&target_name)),
            "main_menu").await;
    } else {
        let bars = "⚠️".repeat(count as usize) + &"▪️".repeat(3 - count as usize);
        mod_msg(bot, chat_id,
            &format!("⚠️ <b>Warning {}/3</b> issued to <a href=\"tg://user?id={}\">{}</a>\n\n{}\n\n<i>3 warnings = auto-ban.</i>",
                count, target_id, html_escape(&target_name), bars),
            "main_menu").await;
    }
}

// ════════════════════════════════════════════════════════════════
//  /unwarn — Remove one warning from a user
// ════════════════════════════════════════════════════════════════

pub async fn handle_unwarn(
    bot: &Bot,
    chat_id: i64,
    reply_user_id: Option<i64>,
    reply_user_name: Option<&str>,
    warn_store: &WarnStore,
) {
    let (target_id, target_name) = match (reply_user_id, reply_user_name) {
        (Some(id), Some(name)) => (id, name.to_string()),
        _ => {
            mod_msg(bot, chat_id,
                "⚠️ <b>Usage:</b> Reply to a message with <code>/unwarn</code>",
                "main_menu").await;
            return;
        }
    };

    let count = {
        let mut store = warn_store.lock().unwrap();
        let entry = store.entry((chat_id, target_id)).or_insert(0);
        if *entry > 0 { *entry -= 1; }
        *entry
    };

    mod_msg(bot, chat_id,
        &format!("✅ Warning removed from <a href=\"tg://user?id={}\">{}</a>\n\nCurrent warnings: <b>{}/3</b>",
            target_id, html_escape(&target_name), count),
        "main_menu").await;
}

// ════════════════════════════════════════════════════════════════
//  /warns — Check how many warnings a user has
// ════════════════════════════════════════════════════════════════

pub async fn handle_warns(
    bot: &Bot,
    chat_id: i64,
    reply_user_id: Option<i64>,
    reply_user_name: Option<&str>,
    warn_store: &WarnStore,
) {
    let (target_id, target_name) = match (reply_user_id, reply_user_name) {
        (Some(id), Some(name)) => (id, name.to_string()),
        _ => {
            mod_msg(bot, chat_id,
                "⚠️ <b>Usage:</b> Reply to a message with <code>/warns</code>",
                "main_menu").await;
            return;
        }
    };

    let count = {
        let store = warn_store.lock().unwrap();
        *store.get(&(chat_id, target_id)).unwrap_or(&0)
    };

    let bars = "⚠️".repeat(count as usize) + &"▪️".repeat(3 - count.min(3) as usize);
    mod_msg(bot, chat_id,
        &format!("📋 <b>Warnings for</b> <a href=\"tg://user?id={}\">{}</a>: <b>{}/3</b>\n\n{}",
            target_id, html_escape(&target_name), count, bars),
        "main_menu").await;
}

// ════════════════════════════════════════════════════════════════
//  /delete — Delete the replied-to message
// ════════════════════════════════════════════════════════════════

pub async fn handle_delete(
    bot: &Bot,
    chat_id: i64,
    reply_message_id: Option<i64>,
    command_message_id: i64,
) {
    // Delete the command message itself
    let _ = bot.delete_message(ChatId::from(chat_id), command_message_id).await;

    match reply_message_id {
        Some(mid) => {
            match bot.delete_message(ChatId::from(chat_id), mid).await {
                Ok(_) => {} // silent success
                Err(e) => {
                    mod_msg(bot, chat_id,
                        &format!("❌ <b>Delete failed:</b> <code>{}</code>", e),
                        "main_menu").await;
                }
            }
        }
        None => {
            mod_msg(bot, chat_id,
                "⚠️ <b>Usage:</b> Reply to a message with <code>/delete</code>",
                "main_menu").await;
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  /pin — Pin the replied-to message
// ════════════════════════════════════════════════════════════════

pub async fn handle_pin(
    bot: &Bot,
    chat_id: i64,
    reply_message_id: Option<i64>,
) {
    match reply_message_id {
        Some(mid) => {
            match bot.pin_chat_message(ChatId::from(chat_id), mid, None).await {
                Ok(_) => {
                    mod_msg(bot, chat_id,
                        "📌 <b>Message pinned!</b>",
                        "main_menu").await;
                }
                Err(e) => {
                    mod_msg(bot, chat_id,
                        &format!("❌ <b>Pin failed:</b> <code>{}</code>\n\n<i>Bot must be admin with pin rights.</i>", e),
                        "main_menu").await;
                }
            }
        }
        None => {
            mod_msg(bot, chat_id,
                "⚠️ <b>Usage:</b> Reply to a message with <code>/pin</code>",
                "main_menu").await;
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  /unpin — Unpin the current pinned message
// ════════════════════════════════════════════════════════════════

pub async fn handle_unpin(bot: &Bot, chat_id: i64) {
    match bot.unpin_chat_message(ChatId::from(chat_id), None).await {
        Ok(_) => {
            mod_msg(bot, chat_id, "📌 <b>Message unpinned!</b>", "main_menu").await;
        }
        Err(e) => {
            mod_msg(bot, chat_id,
                &format!("❌ <b>Unpin failed:</b> <code>{}</code>", e),
                "main_menu").await;
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  /ro — Set chat to read-only (mute everyone)
//  /unro — Restore normal chat permissions
// ════════════════════════════════════════════════════════════════

pub async fn handle_ro(bot: &Bot, chat_id: i64) {
    let perms = no_perms();
    match bot.set_chat_permissions(ChatId::from(chat_id), perms, None).await {
        Ok(_) => {
            mod_msg(bot, chat_id,
                "🔇 <b>Read-only mode ON</b>\n\nOnly admins can send messages.\nUse <code>/unro</code> to restore.",
                "main_menu").await;
        }
        Err(e) => {
            mod_msg(bot, chat_id,
                &format!("❌ <b>Failed:</b> <code>{}</code>", e),
                "main_menu").await;
        }
    }
}

pub async fn handle_unro(bot: &Bot, chat_id: i64) {
    let perms = all_perms();
    match bot.set_chat_permissions(ChatId::from(chat_id), perms, None).await {
        Ok(_) => {
            mod_msg(bot, chat_id,
                "🔊 <b>Read-only mode OFF</b>\n\nAll members can send messages again.",
                "main_menu").await;
        }
        Err(e) => {
            mod_msg(bot, chat_id,
                &format!("❌ <b>Failed:</b> <code>{}</code>", e),
                "main_menu").await;
        }
    }
}

// ── HTML escape ───────────────────────────────────────────────────────────────

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

// ── Moderation help text (used by /modhelp) ──────────────────────────────────

pub async fn handle_mod_help(bot: &Bot, chat_id: i64) {
    let text = "🛡️ <b>Moderation Commands</b>\n\n\
        All commands below work by <b>replying</b> to the target user's message.\n\
        Bot must be admin with appropriate rights.\n\n\
        <b>👤 User Actions</b>\n\
        /ban — Ban forever\n\
        /ban 7d — Ban for 7 days\n\
        /ban 2h — Ban for 2 hours\n\
        /ban 30m — Ban for 30 minutes\n\
        /unban — Unban a user\n\
        /kick — Remove (can rejoin)\n\n\
        <b>🔇 Mute</b>\n\
        /mute — Mute forever\n\
        /mute 1h — Mute for 1 hour\n\
        /mute 30m — Mute for 30 minutes\n\
        /unmute — Restore permissions\n\n\
        <b>⚠️ Warnings</b>\n\
        /warn — Warn user (auto-ban at 3)\n\
        /unwarn — Remove one warning\n\
        /warns — Check user's warnings\n\n\
        <b>💬 Messages</b>\n\
        /delete — Delete replied message\n\
        /pin — Pin replied message\n\
        /unpin — Unpin current message\n\n\
        <b>🌐 Chat</b>\n\
        /ro — Read-only mode (mutes everyone)\n\
        /unro — Restore normal chat\n\n\
        <b>⏱ Duration format</b>\n\
        <code>Nd</code> = days, <code>Nh</code> = hours, <code>Nm</code> = minutes\n\
        <i>Example: /ban 7d, /mute 2h, /ban 30m</i>";

    let kb = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![btn("⬅️ Main Menu", "main_menu")]],
    };
    let p = SendMessageParams::new()
        .parse_mode("HTML")
        .reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}
