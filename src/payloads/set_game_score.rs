// This file is auto generated by `cg` <https://github.com/teloxide/cg> (9a82143).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::Message;

impl_payload! {
    /// Use this method to set the score of the specified user in a game. On success, returns the edited [`Message`]. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
    ///
    /// See also: [`SetGameScoreInline`](crate::payloads::SetGameScoreInline)
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetGameScore (SetGameScoreSetters) => Message {
        required {
            /// User identifier
            pub user_id: u32,
            /// New score
            pub score: u64,
            /// Unique identifier for the target chat
            pub chat_id: u32,
            /// Identifier of the message to edit
            pub message_id: i64,
        }
        optional {
            /// Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
            pub force: bool,
            /// Pass True, if the game message should not be automatically edited to include the current scoreboard
            pub disable_edit_message: bool,
        }
    }
}
