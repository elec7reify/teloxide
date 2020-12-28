// This file is auto generated by `cg` <https://github.com/teloxide/cg> (9a82143).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, True};

impl_payload! {
    /// Use this method to kick a user from a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless [unbanned] first. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns _True_ on success.
    ///
    /// [unbanned]: crate::payloads::UnbanChatMember
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub KickChatMember (KickChatMemberSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Unique identifier of the target user
            pub user_id: i32,
        }
        optional {
            /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever
            pub until_date: u64,
        }
    }
}
