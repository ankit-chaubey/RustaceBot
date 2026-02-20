// ════════════════════════════════════════════════════════════════
//  Rustace Bot — Update Dispatcher
// ════════════════════════════════════════════════════════════════

use tgbotrs::{
    gen_methods::SendMessageParams,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage},
    Bot, ReplyMarkup,
};

use crate::handlers::{
    admin,
    broadcast,
    callbacks::handle_callback,
    commands::*,
    filters::{self, FilterStore},
    inline::handle_inline_query,
    moderation::{self, WarnStore},
    notes::{self, NoteStore},
};

pub struct Stores {
    pub warn:   WarnStore,
    pub filter: FilterStore,
    pub note:   NoteStore,
}

pub async fn dispatch(bot: Bot, update: tgbotrs::types::Update, stores: Stores) {
    log::debug!("update_id={}", update.update_id);

    // ── Message ──────────────────────────────────────────────────────────────
    if let Some(msg) = update.message {
        let chat_id    = msg.chat.id;
        let user_id    = msg.from.as_ref().map(|u| u.id).unwrap_or(0);
        let first_name = msg.from.as_ref().map(|u| u.first_name.as_str()).unwrap_or("there");
        let msg_id     = msg.message_id;

        // Reply-to info (used by moderation + admin commands)
        let reply_user_id:   Option<i64>    = msg.reply_to_message.as_ref().and_then(|r| r.from.as_ref()).map(|u| u.id);
        let reply_user_name: Option<String> = msg.reply_to_message.as_ref().and_then(|r| r.from.as_ref()).map(|u| u.first_name.clone());
        let reply_msg_id:    Option<i64>    = msg.reply_to_message.as_ref().map(|r| r.message_id);
        let chat_type       = msg.chat.r#type.as_str().to_owned(); // "private" | "group" | "supergroup" | "channel"
        let is_private      = chat_type == "private";
        let msg_date        = msg.date;

        if let Some(ref text) = msg.text {
            // ── Filters & Notes auto-triggers (before command parsing) ────────
            // #notename shortcut
            if notes::check_hashtag_note(&bot, chat_id, text, &stores.note).await { return; }

            let mut parts   = text.split_whitespace();
            let command_raw = parts.next().unwrap_or("");
            let command     = command_raw.split('@').next().unwrap_or(command_raw);
            // Collect remaining words as a slice for multi-arg commands
            let args_vec: Vec<&str> = parts.collect();
            let args: &[&str]       = &args_vec;
            let arg0                = args.first().copied(); // convenience: first arg

            // Text after command as full string (for /send, /post, /note etc)
            let rest_of_line: &str = text[command_raw.len()..].trim();

            match command {
                // ── Core ──────────────────────────────────────────────────
                "/start" | "/menu" => handle_start(&bot, chat_id, first_name).await,
                "/help"            => handle_help(&bot, chat_id).await,
                "/about"           => handle_about(&bot, chat_id, None).await,
                "/library"         => handle_library(&bot, chat_id, None).await,
                "/textstyles"      => handle_text_styles(&bot, chat_id, None).await,
                "/stats"           => handle_stats(&bot, chat_id).await,

                // ── Fun ───────────────────────────────────────────────────
                "/dice"       => handle_dice(&bot, chat_id, "🎲").await,
                "/darts"      => handle_dice(&bot, chat_id, "🎯").await,
                "/bowling"    => handle_dice(&bot, chat_id, "🎳").await,
                "/basketball" => handle_dice(&bot, chat_id, "🏀").await,
                "/football"   => handle_dice(&bot, chat_id, "⚽").await,
                "/slots"      => handle_dice(&bot, chat_id, "🎰").await,
                "/fact"       => handle_fact(&bot, chat_id).await,
                "/joke"       => handle_joke(&bot, chat_id).await,
                "/magic8"     => handle_magic8(&bot, chat_id).await,
                "/coinflip"   => handle_coinflip(&bot, chat_id).await,

                // ── Media demos ───────────────────────────────────────────
                "/photo"      => handle_photo(&bot, chat_id).await,
                "/animation"  => handle_animation(&bot, chat_id).await,
                "/location"   => handle_location(&bot, chat_id).await,
                "/venue"      => handle_venue(&bot, chat_id).await,
                "/contact"    => handle_contact(&bot, chat_id).await,
                "/poll"       => handle_poll(&bot, chat_id).await,

                // ── Info ──────────────────────────────────────────────────
                "/botinfo"     => handle_bot_info(&bot, chat_id, None).await,
                "/webhookinfo" => handle_webhook_info(&bot, chat_id, None).await,
                "/membercount" => handle_member_count(&bot, chat_id, None).await,
                "/admins"      => handle_admins(&bot, chat_id, None).await,
                "/invitelink"  => handle_invite_link(&bot, chat_id, None).await,
                "/mycommands"  => handle_my_commands(&bot, chat_id, None).await,
                "/myprofile"   => handle_my_profile(&bot, chat_id, user_id).await,

                // ── Admin commands ────────────────────────────────────────
                "/promote"  => admin::handle_promote(&bot, chat_id, reply_user_id, reply_user_name.as_deref(), args).await,
                "/demote"   => admin::handle_demote(&bot, chat_id, reply_user_id, reply_user_name.as_deref(), args).await,
                "/title"    => admin::handle_title(&bot, chat_id, reply_user_id, reply_user_name.as_deref(), args).await,
                "/userinfo" | "/whois" => admin::handle_userinfo(&bot, chat_id, reply_user_id, reply_user_name.as_deref(), arg0).await,

                // ── Moderation ────────────────────────────────────────────
                "/modhelp" => moderation::handle_mod_help(&bot, chat_id).await,
                "/ban"     => moderation::handle_ban(&bot, chat_id, reply_user_id, reply_user_name.as_deref(), arg0).await,
                "/unban"   => moderation::handle_unban(&bot, chat_id, reply_user_id, reply_user_name.as_deref()).await,
                "/kick"    => moderation::handle_kick(&bot, chat_id, reply_user_id, reply_user_name.as_deref()).await,
                "/mute"    => moderation::handle_mute(&bot, chat_id, reply_user_id, reply_user_name.as_deref(), arg0).await,
                "/unmute"  => moderation::handle_unmute(&bot, chat_id, reply_user_id, reply_user_name.as_deref()).await,
                "/warn"    => moderation::handle_warn(&bot, chat_id, reply_user_id, reply_user_name.as_deref(), &stores.warn).await,
                "/unwarn"  => moderation::handle_unwarn(&bot, chat_id, reply_user_id, reply_user_name.as_deref(), &stores.warn).await,
                "/warns"   => moderation::handle_warns(&bot, chat_id, reply_user_id, reply_user_name.as_deref(), &stores.warn).await,
                "/delete" | "/del" => moderation::handle_delete(&bot, chat_id, reply_msg_id, msg_id).await,
                "/pin"     => moderation::handle_pin(&bot, chat_id, reply_msg_id).await,
                "/unpin"   => moderation::handle_unpin(&bot, chat_id).await,
                "/ro"      => moderation::handle_ro(&bot, chat_id).await,
                "/unro"    => moderation::handle_unro(&bot, chat_id).await,

                // ── Filters ───────────────────────────────────────────────
                "/filter"    => filters::handle_set_filter(&bot, chat_id, args, &stores.filter).await,
                "/delfilter" => filters::handle_del_filter(&bot, chat_id, arg0, &stores.filter).await,
                "/filters"   => filters::handle_list_filters(&bot, chat_id, &stores.filter).await,

                // ── Notes ─────────────────────────────────────────────────
                "/note"    => notes::handle_save_note(&bot, chat_id, args, &stores.note).await,
                "/get"     => notes::handle_get_note(&bot, chat_id, arg0, &stores.note).await,
                "/notes"   => notes::handle_list_notes(&bot, chat_id, &stores.note).await,
                "/delnote" => notes::handle_del_note(&bot, chat_id, arg0, &stores.note).await,

                // ── Send / Post / Media ───────────────────────────────────
                "/send"     => broadcast::handle_send(&bot, chat_id, rest_of_line).await,
                "/post"     => broadcast::handle_post(&bot, chat_id, rest_of_line).await,
                "/img"      => broadcast::handle_img(&bot, chat_id, args).await,
                "/vid"      => broadcast::handle_vid(&bot, chat_id, args).await,
                "/aud"      => broadcast::handle_aud(&bot, chat_id, args).await,
                "/doc"      => broadcast::handle_doc(&bot, chat_id, args).await,
                "/buttons"  => broadcast::handle_buttons_showcase(&bot, chat_id).await,
                "/sendhelp" => broadcast::handle_send_help(&bot, chat_id).await,

                // ── Ping ──────────────────────────────────────────────────
                "/ping" => handle_ping(&bot, chat_id, msg_date).await,

                // ── System ────────────────────────────────────────────────
                "/setcommands" => {
                    match register_commands(&bot).await {
                        Ok(_)  => { let _ = bot.send_message(chat_id, "✅ Commands registered!", None).await; }
                        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Error: {}", e), None).await; }
                    }
                }
                "/deletecommands" => {
                    match bot.delete_my_commands(None).await {
                        Ok(_)  => { let _ = bot.send_message(chat_id, "✅ Commands deleted!", None).await; }
                        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Error: {}", e), None).await; }
                    }
                }
                "/deletewebhook" => {
                    match bot.delete_webhook(None).await {
                        Ok(_)  => { let _ = bot.send_message(chat_id, "✅ Webhook deleted!", None).await; }
                        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Error: {}", e), None).await; }
                    }
                }

                // ── Unknown command ───────────────────────────────────────
                // Stay silent in groups/supergroups to avoid spamming.
                _ if command.starts_with('/') => {
                    if is_private {
                        let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![
                            InlineKeyboardButton { text: "📋 Menu".into(), callback_data: Some("main_menu".into()), ..Default::default() },
                            InlineKeyboardButton { text: "📖 Help".into(), callback_data: Some("help_cb".into()), ..Default::default() },
                        ]]};
                        let p = SendMessageParams::new().parse_mode("HTML")
                            .reply_markup(ReplyMarkup::InlineKeyboard(kb));
                        let _ = bot.send_message(chat_id,
                            format!("❓ Unknown command: <code>{}</code>\n\nUse /help to see all commands.", command),
                            Some(p)).await;
                    }
                    // In groups: silently ignore unknown commands
                }

                // ── Plain text: check filters, then echo only in private ──
                _ => {
                    if !filters::check_filters(&bot, chat_id, text, &stores.filter).await {
                        if is_private {
                            handle_text_echo(&bot, chat_id, text, first_name).await;
                        }
                        // In groups: silently ignore unmatched plain text
                    }
                }
            }

        // ── Non-text messages ─────────────────────────────────────────────────
        } else if let Some(sticker) = msg.sticker {
            let p = SendMessageParams::new().parse_mode("HTML");
            let _ = bot.send_message(chat_id, format!(
                "🎭 <b>Sticker!</b>\n\n<b>File ID:</b> <code>{}</code>\n<b>Set:</b> {}\n<b>Emoji:</b> {}\n\n<i>Use with <code>bot.send_sticker()</code></i>",
                sticker.file_id,
                sticker.set_name.as_deref().unwrap_or("Unknown"),
                sticker.emoji.as_deref().unwrap_or("—"),
            ), Some(p)).await;
        } else if let Some(photos) = msg.photo {
            if let Some(p) = photos.last() {
                let pm = SendMessageParams::new().parse_mode("HTML");
                let _ = bot.send_message(chat_id, format!(
                    "📸 <b>Photo!</b>\n\n<b>File ID:</b> <code>{}</code>\n<b>Size:</b> {}×{}\n\n<i>Use with <code>bot.send_photo()</code></i>",
                    p.file_id, p.width, p.height,
                ), Some(pm)).await;
            }
        } else if let Some(doc) = msg.document {
            let p = SendMessageParams::new().parse_mode("HTML");
            let _ = bot.send_message(chat_id, format!(
                "📁 <b>Document!</b>\n\n<b>Name:</b> {}\n<b>File ID:</b> <code>{}</code>\n<b>MIME:</b> {}\n\n<i>Use <code>bot.get_file(file_id)</code> to download.</i>",
                doc.file_name.as_deref().unwrap_or("Unknown"),
                doc.file_id,
                doc.mime_type.as_deref().unwrap_or("Unknown"),
            ), Some(p)).await;
        } else if let Some(loc) = msg.location {
            let p = SendMessageParams::new().parse_mode("HTML");
            let _ = bot.send_message(chat_id, format!(
                "📍 <b>Location!</b>\n\n<b>Lat:</b> {}\n<b>Lon:</b> {}\n\n<code>bot.send_location(chat_id, {}, {}, None)</code>",
                loc.latitude, loc.longitude, loc.latitude, loc.longitude,
            ), Some(p)).await;
        } else if let Some(contact) = msg.contact {
            let p = SendMessageParams::new().parse_mode("HTML");
            let _ = bot.send_message(chat_id, format!(
                "📞 <b>Contact!</b>\n\n<b>Name:</b> {} {}\n<b>Phone:</b> <code>{}</code>",
                contact.first_name,
                contact.last_name.as_deref().unwrap_or(""),
                contact.phone_number,
            ), Some(p)).await;
        }
        return;
    }

    // ── Callback Query ────────────────────────────────────────────────────────
    if let Some(cq) = update.callback_query {
        let data       = cq.data.as_deref().unwrap_or("").to_string();
        let user_id    = cq.from.id;
        let first_name = cq.from.first_name.clone();
        let (chat_id, message_id) = match &cq.message {
            Some(m) => match m.as_ref() {
                MaybeInaccessibleMessage::Message(msg)              => (msg.chat.id, msg.message_id),
                MaybeInaccessibleMessage::InaccessibleMessage(im)  => (im.chat.id, im.message_id),
            },
            None => return,
        };

        // Handle showcase button callbacks gracefully
        match data.as_str() {
            "btn_color" | "btn_shape" => {
                let _ = bot.answer_callback_query(&cq.id, Some(
                    tgbotrs::gen_methods::AnswerCallbackQueryParams::new()
                        .text("🎨 Pretty button clicked! 🦀")
                        .show_alert(false)
                )).await;
                return;
            }
            "alert_demo" => {
                let _ = bot.answer_callback_query(&cq.id, Some(
                    tgbotrs::gen_methods::AnswerCallbackQueryParams::new()
                        .text("🚨 This is a popup ALERT!\nBuilt with tgbotrs AnswerCallbackQueryParams.")
                        .show_alert(true)
                )).await;
                return;
            }
            "toast_demo" | "notif_demo" => {
                let _ = bot.answer_callback_query(&cq.id, Some(
                    tgbotrs::gen_methods::AnswerCallbackQueryParams::new()
                        .text("📢 Toast notification! No popup.")
                        .show_alert(false)
                )).await;
                return;
            }
            "cb_url_demo" => {
                let _ = bot.answer_callback_query(&cq.id, Some(
                    tgbotrs::gen_methods::AnswerCallbackQueryParams::new()
                        .text("🔔 Callback received!")
                        .show_alert(false)
                )).await;
                return;
            }
            _ => {}
        }

        handle_callback(&bot, cq.id, chat_id, message_id, &data, user_id, &first_name).await;
        return;
    }

    // ── Inline Query ──────────────────────────────────────────────────────────
    if let Some(iq) = update.inline_query {
        let query = iq.query.clone();
        handle_inline_query(&bot, iq.id.clone(), &query).await;
        return;
    }

    // ── Chosen Inline Result ──────────────────────────────────────────────────
    if let Some(cir) = update.chosen_inline_result {
        log::info!("chosen_inline_result: {} from user {}", cir.result_id, cir.from.id);
        return;
    }

    // ── Shipping Query ────────────────────────────────────────────────────────
    if let Some(sq) = update.shipping_query {
        let _ = bot.answer_shipping_query(&sq.id, true, None).await;
        return;
    }

    // ── Pre-Checkout Query ────────────────────────────────────────────────────
    if let Some(pcq) = update.pre_checkout_query {
        let _ = bot.answer_pre_checkout_query(&pcq.id, true, None).await;
        return;
    }

    // ── Poll / Poll Answer ────────────────────────────────────────────────────
    if let Some(poll) = update.poll         { log::info!("poll: {}", poll.id); return; }
    if let Some(pa)   = update.poll_answer  { log::info!("poll_answer: {:?}", pa.option_ids); return; }

    // ── My Chat Member ────────────────────────────────────────────────────────
    if let Some(mcm) = update.my_chat_member {
        let chat_id = mcm.chat.id;
        let v = serde_json::to_value(&mcm.new_chat_member).unwrap_or_default();
        let status = v.get("status").and_then(|s| s.as_str()).unwrap_or("");
        if status == "member" || status == "administrator" {
            let p = SendMessageParams::new().parse_mode("HTML");
            let _ = bot.send_message(chat_id,
                "🦀 <b>Thanks for adding Rustace!</b>\n\n\
                I'm @RustaceBot — the official showcase bot for \
                <a href=\"https://github.com/ankit-chaubey/tgbotrs\">tgbotrs</a>.\n\n\
                Use /start to explore!", Some(p)).await;
        }
        return;
    }

    // ── Chat Member ───────────────────────────────────────────────────────────
    if let Some(cm) = update.chat_member { log::info!("chat_member: {}", cm.chat.id); return; }

    // ── Chat Join Request ─────────────────────────────────────────────────────
    if let Some(jr) = update.chat_join_request {
        let _ = bot.approve_chat_join_request(jr.chat.id, jr.from.id).await;
        return;
    }

    // ── Reactions & Boosts ────────────────────────────────────────────────────
    if let Some(mr)  = update.message_reaction       { log::info!("reaction in {}", mr.chat.id); return; }
    if let Some(mrc) = update.message_reaction_count { log::info!("reaction_count in {}", mrc.chat.id); return; }
    if let Some(cb)  = update.chat_boost             { log::info!("boost in {}", cb.chat.id); return; }
    if let Some(rcb) = update.removed_chat_boost     { log::info!("boost_removed in {}", rcb.chat.id); return; }
}

// ── Text echo ─────────────────────────────────────────────────────────────────

async fn handle_text_echo(bot: &Bot, chat_id: i64, text: &str, first_name: &str) {
    let lower  = text.to_lowercase();
    let reply  = if lower.contains("rust") || lower.contains("🦀") {
        format!("🦀 <b>Rust fan!</b>\n\nI love Rust too, {}! Try /fact for trivia or /menu to explore!", first_name)
    } else if lower.contains("hello") || lower.contains("hi") || lower.contains("hey") {
        format!("👋 <b>Hey, {}!</b>\n\nI'm Rustace — powered by tgbotrs. Use /start to explore!", first_name)
    } else if lower.contains("help") {
        format!("ℹ️ Use /help to see all commands, {}!", first_name)
    } else if lower.contains("thank") {
        format!("😊 You're welcome, {}! 🦀", first_name)
    } else {
        format!("💬 <code>{}</code>\n\nUse /help or /menu!", html_escape(text))
    };

    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![
        InlineKeyboardButton { text: "📋 Menu".into(), callback_data: Some("main_menu".into()), ..Default::default() },
        InlineKeyboardButton { text: "📖 Help".into(), callback_data: Some("help_cb".into()), ..Default::default() },
    ]]};
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, reply, Some(p)).await;
}

async fn handle_stats(bot: &Bot, chat_id: i64) {
    let text = "📊 <b>Rustace Bot Statistics</b>\n\n\
        <b>Version:</b> 0.1.0\n\
        <b>Library:</b> tgbotrs v0.1.4\n\
        <b>API:</b> Telegram Bot API 9.4\n\
        <b>Methods:</b> 165/165 ✅\n\
        <b>Types:</b> 285/285 ✅\n\n\
        <b>Feature Modules:</b>\n\
        ✅ Core commands & menus\n\
        ✅ Moderation (ban/mute/kick/warn)\n\
        ✅ Admin (promote/demote/title/userinfo)\n\
        ✅ Filters (keyword auto-replies)\n\
        ✅ Notes (#hashtag system)\n\
        ✅ Broadcast (/send /post + buttons)\n\
        ✅ Media (/img /vid /aud /doc)\n\
        ✅ Inline search (method database)\n\
        ✅ Colourful button showcase\n\n\
        <b>Update Types:</b> All 15 handled ✅\n\n\
        <i>Built with ❤️ by Ankit Chaubey</i>";
    let kb = InlineKeyboardMarkup { inline_keyboard: vec![vec![
        InlineKeyboardButton { text: "⬅️ Menu".into(), callback_data: Some("main_menu".into()), ..Default::default() }
    ]]};
    let p = SendMessageParams::new().parse_mode("HTML").reply_markup(ReplyMarkup::InlineKeyboard(kb));
    let _ = bot.send_message(chat_id, text, Some(p)).await;
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
