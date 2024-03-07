use teloxide::prelude::*;
use crate::utils::admins::{is_group, require_admin};

#[allow(unused)]
pub async fn pin_msg(b: &Bot, m: &Message) -> ResponseResult<()> {
    if !(is_group(b, m).await) {
        return Ok(());
    }

    if !(require_admin(b, m).await) {
        return Ok(());
    }

    if m.reply_to_message().is_none() {
        b.send_message(m.chat.id, "Reply to a message to pin")
            .reply_to_message_id(m.id)
            .send()
            .await;
        return Ok(());
    } else {
        b.pin_chat_message(m.chat.id, m.reply_to_message().unwrap().id).await;
        b.delete_message(m.chat.id, m.id).await;
    }
    Ok(())
}

#[allow(unused)]
pub async fn unpin_msg(b: &Bot, m: &Message) -> ResponseResult<()> {
    if !(is_group(b, m).await) {
        return Ok(());
    }

    if !(require_admin(b, m).await) {
        return Ok(());
    }

    if m.reply_to_message().is_none() {
        b.send_message(m.chat.id, "Reply to a pinned message to unpin")
            .reply_to_message_id(m.id)
            .send()
            .await;
        return Ok(());
    } else {
        b.unpin_chat_message(m.chat.id)
            .message_id(m.reply_to_message().unwrap().id)
            .send()
            .await;
        b.delete_message(m.chat.id, m.id).await;
    }
    Ok(())
}

#[allow(unused)]
pub async fn unpin_all_msg(b: &Bot, m: &Message) -> ResponseResult<()> {
    if !(is_group(b, m).await) {
        return Ok(());
    }

    if !(require_admin(b, m).await) {
        return Ok(());
    }

    b.unpin_all_chat_messages(m.chat.id).await;
    b.send_message(m.chat.id, "Unpinned all the pinned messages")
        .reply_to_message_id(m.id)
        .send()
        .await;
    Ok(())
}