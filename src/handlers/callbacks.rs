// ════════════════════════════════════════════════════════════════
//  Rustace Bot — Callback Query Handler
//  Showcases: answer_callback_query (alert/toast/url), edit_message_text
// ════════════════════════════════════════════════════════════════

use tgbotrs::{
    gen_methods::{AnswerCallbackQueryParams, EditMessageTextParams, SendMessageParams},
    types::InlineKeyboardMarkup,
    Bot, ChatId, ReplyMarkup,
};

use super::commands::*;

pub async fn handle_callback(
    bot: &Bot,
    cq_id: String,
    chat_id: i64,
    message_id: i64,
    data: &str,
    user_id: i64,
    first_name: &str,
) {
    match data {
        // ── Navigation ──────────────────────────────────────────────────
        "main_menu" => {
            ack(bot, &cq_id, None, false).await;
            edit_text(bot, chat_id, message_id,
                &format!(
                    "🦀 <b>Rustace Main Menu</b>\n\n\
                    Welcome back, <b>{}</b>! Choose a category:",
                    first_name
                ),
                if let ReplyMarkup::InlineKeyboard(kb) = main_menu() { kb } else { InlineKeyboardMarkup { inline_keyboard: vec![] } },
            ).await;
        }

        "about" => {
            ack(bot, &cq_id, None, false).await;
            handle_about(bot, chat_id).await;
        }

        "library" => {
            ack(bot, &cq_id, None, false).await;
            handle_library(bot, chat_id).await;
        }

        "stats_info" => {
            ack(bot, &cq_id, None, false).await;
            handle_stats_info(bot, chat_id).await;
        }

        // ── Fun Menu ────────────────────────────────────────────────────
        "fun_menu" => {
            ack(bot, &cq_id, None, false).await;
            edit_text(bot, chat_id, message_id,
                "🎮 <b>Fun &amp; Games</b>\n\nChoose an activity!",
                fun_menu(),
            ).await;
        }
        "dice"       => { ack(bot, &cq_id, Some("🎲 Rolling..."), false).await; handle_dice(bot, chat_id, "🎲").await; }
        "darts"      => { ack(bot, &cq_id, Some("🎯 Throwing..."), false).await; handle_dice(bot, chat_id, "🎯").await; }
        "bowling"    => { ack(bot, &cq_id, Some("🎳 Bowling!"), false).await; handle_dice(bot, chat_id, "🎳").await; }
        "basketball" => { ack(bot, &cq_id, Some("🏀 Shooting!"), false).await; handle_dice(bot, chat_id, "🏀").await; }
        "football"   => { ack(bot, &cq_id, Some("⚽ Kicking!"), false).await; handle_dice(bot, chat_id, "⚽").await; }
        "slots"      => { ack(bot, &cq_id, Some("🎰 Spinning..."), false).await; handle_dice(bot, chat_id, "🎰").await; }
        "fact"       => { ack(bot, &cq_id, None, false).await; handle_fact(bot, chat_id).await; }
        "joke"       => { ack(bot, &cq_id, None, false).await; handle_joke(bot, chat_id).await; }
        "magic8"     => { ack(bot, &cq_id, None, false).await; handle_magic8(bot, chat_id).await; }
        "coinflip"   => { ack(bot, &cq_id, None, false).await; handle_coinflip(bot, chat_id).await; }

        // ── API Menu ────────────────────────────────────────────────────
        "api_menu" => {
            ack(bot, &cq_id, None, false).await;
            edit_text(bot, chat_id, message_id,
                "📡 <b>API Showcase</b>\n\nExplore Telegram Bot API methods in action.",
                api_menu(),
            ).await;
        }
        "webhook_info" => { ack(bot, &cq_id, Some("📡 Fetching..."), false).await; handle_webhook_info(bot, chat_id).await; }
        "bot_details" => { ack(bot, &cq_id, Some("🤖 Getting info..."), false).await; handle_bot_info(bot, chat_id).await; }
        "botinfo"     => { ack(bot, &cq_id, Some("🤖 Getting info..."), false).await; handle_bot_info(bot, chat_id).await; }
        "member_count" => { ack(bot, &cq_id, Some("👥 Counting..."), false).await; handle_member_count(bot, chat_id).await; }
        "admins"      => { ack(bot, &cq_id, Some("👑 Fetching..."), false).await; handle_admins(bot, chat_id).await; }
        "invite_link" => { ack(bot, &cq_id, Some("🔗 Generating..."), false).await; handle_invite_link(bot, chat_id).await; }
        "my_commands" => { ack(bot, &cq_id, Some("📋 Fetching..."), false).await; handle_my_commands(bot, chat_id).await; }
        "my_profile"  => { ack(bot, &cq_id, Some("👤 Fetching..."), false).await; handle_my_profile(bot, chat_id, user_id).await; }
        "stars"       => { ack(bot, &cq_id, Some("⭐ Checking..."), false).await; handle_star_balance(bot, chat_id).await; }

        // ── Tools Menu ──────────────────────────────────────────────────
        "tools_menu" => {
            ack(bot, &cq_id, None, false).await;
            edit_text(bot, chat_id, message_id,
                "🛠 <b>Tools &amp; Interactive Features</b>",
                tools_menu(),
            ).await;
        }
        "location"   => { ack(bot, &cq_id, Some("📍 Sending..."), false).await; handle_location(bot, chat_id).await; }
        "venue"      => { ack(bot, &cq_id, Some("🏢 Sending..."), false).await; handle_venue(bot, chat_id).await; }
        "contact"    => { ack(bot, &cq_id, Some("📞 Sending..."), false).await; handle_contact(bot, chat_id).await; }
        "poll"       => { ack(bot, &cq_id, Some("📊 Creating..."), false).await; handle_poll(bot, chat_id).await; }
        "text_styles" => { ack(bot, &cq_id, None, false).await; handle_text_styles(bot, chat_id).await; }
        "countdown"  => { ack(bot, &cq_id, None, false).await; handle_countdown_info(bot, chat_id).await; }
        "checklist"  => { ack(bot, &cq_id, None, false).await; handle_checklist_info(bot, chat_id).await; }
        "webapp_info" => { ack(bot, &cq_id, None, false).await; handle_webapp_info(bot, chat_id).await; }

        // ── Media Menu ──────────────────────────────────────────────────
        "media_menu" => {
            ack(bot, &cq_id, None, false).await;
            edit_text(bot, chat_id, message_id,
                "💬 <b>Media Demo</b>\n\nExplore media types in tgbotrs.",
                media_menu(),
            ).await;
        }
        "send_photo"      => { ack(bot, &cq_id, Some("🖼 Demo..."), false).await; handle_photo(bot, chat_id).await; }
        "send_animation"  => { ack(bot, &cq_id, Some("🎬 Demo..."), false).await; handle_animation(bot, chat_id).await; }
        "audio_info"      => { ack(bot, &cq_id, None, false).await; handle_audio_info(bot, chat_id).await; }
        "video_info"      => { ack(bot, &cq_id, None, false).await; handle_video_info(bot, chat_id).await; }
        "voice_info"      => { ack(bot, &cq_id, None, false).await; handle_voice_info(bot, chat_id).await; }
        "doc_info"        => { ack(bot, &cq_id, None, false).await; handle_doc_info(bot, chat_id).await; }
        "sticker_info"    => { ack(bot, &cq_id, None, false).await; handle_sticker_info(bot, chat_id).await; }
        "media_group_info" => { ack(bot, &cq_id, None, false).await; handle_media_group_info(bot, chat_id).await; }

        // ── Alerts Demo ─────────────────────────────────────────────────
        "alerts_menu" => {
            ack(bot, &cq_id, None, false).await;
            edit_text(bot, chat_id, message_id,
                "🔔 <b>Callback Query Alerts</b>\n\n\
                These buttons demo different <code>answer_callback_query</code> behaviours:",
                alerts_menu(),
            ).await;
        }
        "alert_demo" => {
            // Popup alert (modal dialog)
            ack(bot, &cq_id,
                Some("🚨 This is a popup alert!\nanswer_callback_query(show_alert=true)"),
                true).await;
        }
        "notif_demo" => {
            ack(bot, &cq_id, Some("📢 This is a toast notification at the top!"), false).await;
        }
        "cb_url_demo" => {
            let params = AnswerCallbackQueryParams::new()
                .text("Opening tgbotrs on GitHub...".to_string())
                .url("https://github.com/ankit-chaubey/tgbotrs".to_string());
            let _ = bot.answer_callback_query(&cq_id, Some(params)).await;
        }
        "toast_demo" => {
            ack(bot, &cq_id, Some("🍞 Toast — short notification, no popup."), false).await;
        }

        // ── Unknown ─────────────────────────────────────────────────────
        unknown => {
            ack(bot, &cq_id, Some(&format!("Unknown: {}", unknown)), false).await;
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

async fn ack(bot: &Bot, cq_id: &str, text: Option<&str>, show_alert: bool) {
    let mut params = AnswerCallbackQueryParams::new();
    if let Some(t) = text { params = params.text(t.to_string()); }
    if show_alert { params = params.show_alert(true); }
    let _ = bot.answer_callback_query(cq_id, Some(params)).await;
}

async fn edit_text(bot: &Bot, chat_id: i64, message_id: i64, text: &str, kb: InlineKeyboardMarkup) {
    let params = EditMessageTextParams::new()
        .chat_id(ChatId::from(chat_id))
        .message_id(message_id)
        .parse_mode("HTML")
        .reply_markup(kb);
    let _ = bot.edit_message_text(text, Some(params)).await;
}

// ── Additional callback handlers ──────────────────────────────────────────────

async fn handle_star_balance(bot: &Bot, chat_id: i64) {
    use tgbotrs::types::InlineKeyboardButton;
    let (text, kb) = match bot.get_my_star_balance().await {
        Ok(bal) => (
            format!("⭐ <b>Bot Star Balance</b>\n\n\
                Balance: <b>{} Stars</b>\n\n\
                <code>bot.get_my_star_balance()</code>", bal.amount),
            InlineKeyboardMarkup { inline_keyboard: vec![vec![
                InlineKeyboardButton { text: "⬅️ API Menu".into(), callback_data: Some("api_menu".into()), ..Default::default() }
            ]] },
        ),
        Err(e) => (
            format!("⭐ <b>get_my_star_balance</b>\n\nReturns bot's Telegram Star balance.\nError: <code>{}</code>", e),
            InlineKeyboardMarkup { inline_keyboard: vec![vec![
                InlineKeyboardButton { text: "⬅️ API Menu".into(), callback_data: Some("api_menu".into()), ..Default::default() }
            ]] },
        ),
    };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

async fn handle_countdown_info(bot: &Bot, chat_id: i64) {
    use tgbotrs::types::InlineKeyboardButton;
    let text = "⏱️ <b>Live Location</b>\n\n\
        tgbotrs supports live location messages:\n\n\
        <code>send_location(chat_id, lat, lon, params)</code>\n\
        with <code>live_period</code> → live location\n\n\
        <code>edit_message_live_location(lat, lon, params)</code>\n\
        → Update position in real-time\n\n\
        <code>stop_message_live_location(params)</code>\n\
        → Stop sharing\n\n\
        <i>Live period: 60–86400 seconds</i>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![
        InlineKeyboardButton { text: "⬅️ Tools".into(), callback_data: Some("tools_menu".into()), ..Default::default() }
    ]] };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

async fn handle_checklist_info(bot: &Bot, chat_id: i64) {
    use tgbotrs::types::InlineKeyboardButton;
    let text = "🎯 <b>Checklist Messages</b>\n\n\
        New in Telegram Bot API 9.4!\n\n\
        <code>bot.send_checklist(chat_id, title, tasks, params)</code>\n\
        → Send an interactive checklist\n\n\
        <code>bot.edit_message_checklist(...)</code>\n\
        → Edit an existing checklist\n\n\
        <b>Use cases:</b> To-do lists, shopping lists, feature tracking\n\n\
        <i>Exclusive to tgbotrs v0.1.4!</i>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![
        InlineKeyboardButton { text: "⬅️ Tools".into(), callback_data: Some("tools_menu".into()), ..Default::default() }
    ]] };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

async fn handle_stats_info(bot: &Bot, chat_id: i64) {
    use tgbotrs::types::InlineKeyboardButton;
    let text = "📊 <b>Rustace Bot Statistics</b>\n\n\
        <b>Version:</b> 0.1.0\n\
        <b>Library:</b> tgbotrs v0.1.4\n\
        <b>API:</b> Telegram Bot API 9.4\n\
        <b>Methods:</b> 165/165 (100%)\n\
        <b>Types:</b> 285/285 (100%)\n\n\
        <b>Update types handled:</b>\n\
        ✅ message | ✅ callback_query | ✅ inline_query\n\
        ✅ chosen_inline_result | ✅ shipping_query\n\
        ✅ pre_checkout_query | ✅ poll | ✅ poll_answer\n\
        ✅ my_chat_member | ✅ chat_member\n\
        ✅ chat_join_request | ✅ message_reaction\n\
        ✅ message_reaction_count | ✅ chat_boost\n\
        ✅ removed_chat_boost\n\n\
        <b>Modes:</b> Long-polling ✅ | Webhook ✅\n\n\
        <i>Built with ❤️ by Ankit Chaubey</i>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![
        InlineKeyboardButton { text: "⬅️ Main Menu".into(), callback_data: Some("main_menu".into()), ..Default::default() }
    ]] };
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}
