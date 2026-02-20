// ════════════════════════════════════════════════════════════════
//  Rustace Bot — Admin Handlers
//  /promote  /demote  /title  /userinfo
// ════════════════════════════════════════════════════════════════

use tgbotrs::{
    gen_methods::{PromoteChatMemberParams, SendMessageParams},
    types::InlineKeyboardMarkup,
    Bot, ChatId, ReplyMarkup,
};
use super::commands::btn;

pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

async fn reply(bot: &Bot, chat_id: i64, text: &str) {
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Menu", "main_menu")]] };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

fn resolve<'a>(
    reply_id: Option<i64>, reply_name: Option<&'a str>, args: &'a [&'a str],
) -> Option<(i64, String, &'a [&'a str])> {
    if let Some(id) = reply_id {
        return Some((id, reply_name.unwrap_or("User").to_string(), args));
    }
    if let Some(&first) = args.first() {
        if let Ok(id) = first.parse::<i64>() {
            return Some((id, first.to_string(), &args[1..]));
        }
    }
    None
}

// ── /promote [user_id] [Title] ────────────────────────────────────────────────

pub async fn handle_promote(
    bot: &Bot, chat_id: i64,
    reply_user_id: Option<i64>, reply_user_name: Option<&str>, args: &[&str],
) {
    let Some((tid, tname, rest)) = resolve(reply_user_id, reply_user_name, args) else {
        reply(bot, chat_id,
            "⚠️ <b>Usage:</b>\n\
            • Reply: <code>/promote</code> or <code>/promote 🛡️ Guard</code>\n\
            • By ID: <code>/promote 123456789 🛡️ Guard</code>").await;
        return;
    };
    let custom_title: Option<String> = if rest.is_empty() { None } else { Some(rest.join(" ")) };

    let params = PromoteChatMemberParams::new()
        .can_manage_chat(true).can_delete_messages(true).can_manage_video_chats(true)
        .can_restrict_members(true).can_invite_users(true).can_change_info(true)
        .can_pin_messages(true).can_post_stories(true).can_edit_stories(true).can_delete_stories(true);

    match bot.promote_chat_member(ChatId::from(chat_id), tid, Some(params)).await {
        Ok(_) => {
            let title_line = if let Some(ref t) = custom_title {
                let _ = bot.set_chat_administrator_custom_title(ChatId::from(chat_id), tid, t.clone()).await;
                format!("\n🏷️ <b>Title:</b> <i>{}</i>", html_escape(t))
            } else { String::new() };
            reply(bot, chat_id, &format!(
                "⭐ <b>Promoted!</b>\n\n\
                👤 <a href=\"tg://user?id={tid}\">{name}</a>{title}\n\n\
                ✅ Manage chat · Delete messages · Restrict members\n\
                ✅ Invite users · Change info · Pin messages · Stories",
                tid=tid, name=html_escape(&tname), title=title_line
            )).await;
        }
        Err(e) => reply(bot, chat_id, &format!(
            "❌ <b>Promote failed:</b> <code>{}</code>\n<i>Bot must be admin with promote rights.</i>", e)).await,
    }
}

// ── /demote [user_id] ─────────────────────────────────────────────────────────

pub async fn handle_demote(
    bot: &Bot, chat_id: i64,
    reply_user_id: Option<i64>, reply_user_name: Option<&str>, args: &[&str],
) {
    let Some((tid, tname, _)) = resolve(reply_user_id, reply_user_name, args) else {
        reply(bot, chat_id,
            "⚠️ <b>Usage:</b>\n• Reply: <code>/demote</code>\n• By ID: <code>/demote 123456789</code>").await;
        return;
    };
    let params = PromoteChatMemberParams::new()
        .can_manage_chat(false).can_delete_messages(false).can_manage_video_chats(false)
        .can_restrict_members(false).can_invite_users(false).can_change_info(false)
        .can_pin_messages(false).can_promote_members(false).can_post_stories(false)
        .can_edit_stories(false).can_delete_stories(false);
    match bot.promote_chat_member(ChatId::from(chat_id), tid, Some(params)).await {
        Ok(_) => reply(bot, chat_id, &format!(
            "🔽 <b>Demoted!</b>\n\n\
            👤 <a href=\"tg://user?id={tid}\">{name}</a> is now a regular member.",
            tid=tid, name=html_escape(&tname)
        )).await,
        Err(e) => reply(bot, chat_id, &format!("❌ <b>Demote failed:</b> <code>{}</code>", e)).await,
    }
}

// ── /title [user_id] <Title> ──────────────────────────────────────────────────

pub async fn handle_title(
    bot: &Bot, chat_id: i64,
    reply_user_id: Option<i64>, reply_user_name: Option<&str>, args: &[&str],
) {
    let Some((tid, tname, rest)) = resolve(reply_user_id, reply_user_name, args) else {
        reply(bot, chat_id,
            "⚠️ <b>Usage:</b>\n• Reply: <code>/title 🛡️ Guardian</code>\n• By ID: <code>/title 123456789 🛡️ Guardian</code>").await;
        return;
    };
    if rest.is_empty() {
        reply(bot, chat_id, "⚠️ Provide a title. Example: <code>/title 🛡️ Guardian</code>").await;
        return;
    }
    let title = rest.join(" ");
    if title.chars().count() > 16 {
        reply(bot, chat_id, "⚠️ Title must be max 16 characters.").await;
        return;
    }
    match bot.set_chat_administrator_custom_title(ChatId::from(chat_id), tid, title.clone()).await {
        Ok(_) => reply(bot, chat_id, &format!(
            "🏷️ <b>Title set!</b>\n\n\
            👤 <a href=\"tg://user?id={tid}\">{name}</a>\n\
            🔖 <i>{title}</i>",
            tid=tid, name=html_escape(&tname), title=html_escape(&title)
        )).await,
        Err(e) => reply(bot, chat_id, &format!(
            "❌ <b>Failed:</b> <code>{}</code>\n<i>User must already be an admin.</i>", e)).await,
    }
}

// ── /userinfo [user_id | @username | reply] ───────────────────────────────────

pub async fn handle_userinfo(
    bot: &Bot, chat_id: i64,
    reply_user_id: Option<i64>, reply_user_name: Option<&str>, arg: Option<&str>,
) {
    let target_id: i64 = if let Some(id) = reply_user_id {
        id
    } else if let Some(a) = arg {
        if let Ok(id) = a.parse::<i64>() { id }
        else if a.starts_with('@') {
            match bot.get_chat(a).await {
                Ok(info) => info.id,
                Err(_) => {
                    reply(bot, chat_id, &format!(
                        "❌ Could not resolve <code>{}</code>.\n<i>Only works for public users/chats.</i>",
                        html_escape(a))).await;
                    return;
                }
            }
        } else {
            reply(bot, chat_id,
                "⚠️ <b>Usage:</b>\n• Reply to a message: <code>/userinfo</code>\n\
                • By ID: <code>/userinfo 123456789</code>\n• By username: <code>/userinfo @user</code>").await;
            return;
        }
    } else {
        reply(bot, chat_id,
            "⚠️ <b>Usage:</b>\n• Reply to a message: <code>/userinfo</code>\n\
            • By ID: <code>/userinfo 123456789</code>\n• By username: <code>/userinfo @user</code>").await;
        return;
    };

    match bot.get_chat_member(ChatId::from(chat_id), target_id).await {
        Ok(member) => {
            let v       = serde_json::to_value(&member).unwrap_or_default();
            let user    = v.get("user").cloned().unwrap_or_default();
            let first   = user.get("first_name").and_then(|n| n.as_str()).unwrap_or(reply_user_name.unwrap_or("Unknown"));
            let last    = user.get("last_name").and_then(|n| n.as_str()).unwrap_or("");
            let uname   = user.get("username").and_then(|n| n.as_str());
            let is_bot  = user.get("is_bot").and_then(|b| b.as_bool()).unwrap_or(false);
            let premium = user.get("is_premium").and_then(|b| b.as_bool()).unwrap_or(false);
            let status  = v.get("status").and_then(|s| s.as_str()).unwrap_or("unknown");
            let ctitle  = v.get("custom_title").and_then(|s| s.as_str());

            let full_name = if last.is_empty() { html_escape(first) }
                            else { format!("{} {}", html_escape(first), html_escape(last)) };

            let status_label = match status {
                "creator"       => "👑 Creator",
                "administrator" => "⭐ Administrator",
                "member"        => "👤 Member",
                "restricted"    => "🔇 Restricted",
                "left"          => "🚪 Left",
                "kicked"        => "🔨 Banned",
                _               => "❓ Unknown",
            };

            let mut text = format!(
                "👤 <b>User Info</b>\n\n\
                <b>Name:</b> {}\n\
                <b>ID:</b> <code>{}</code>\n\
                <b>Status:</b> {}",
                full_name, target_id, status_label
            );
            if let Some(u) = uname   { text.push_str(&format!("\n<b>Username:</b> @{}", u)); }
            if let Some(t) = ctitle  { text.push_str(&format!("\n<b>Admin Title:</b> <i>{}</i>", html_escape(t))); }
            if premium { text.push_str("\n<b>Premium:</b> 💎"); }
            if is_bot  { text.push_str("\n<b>Type:</b> 🤖 Bot"); }
            text.push_str(&format!("\n\n<a href=\"tg://user?id={}\">📨 Open chat</a>", target_id));

            let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![btn("⬅️ Menu", "main_menu")]] };
            let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
            let _ = bot.send_message(chat_id, text, Some(p)).await;
        }
        Err(e) => reply(bot, chat_id, &format!(
            "❌ <b>Could not get user info:</b> <code>{}</code>\n<i>User must be a member of this chat.</i>", e)).await,
    }
}
