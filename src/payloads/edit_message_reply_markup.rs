// This file is auto generated by `cg` <https://github.com/teloxide/cg> (9a82143).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, InlineKeyboardMarkup, Message};

impl_payload! {
    /// Use this method to edit only the reply markup of messages. On success, the edited Message is returned.
    ///
    /// See also: [`EditMessageMediaInline`](crate::payloads::EditMessageMediaInline)
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub EditMessageReplyMarkup (EditMessageReplyMarkupSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`).
            pub chat_id: ChatId [into],
            /// Identifier of the message to edit
            pub message_id: i32,
        }
        optional {
            /// A JSON-serialized object for an [inline keyboard].
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}
