// This file is auto generated by `cg` <https://github.com/teloxide/cg> (9a82143).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::True;

impl_payload! {
    /// Use this method to move a sticker in a set created by the bot to a specific position. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetStickerPositionInSet (SetStickerPositionInSetSetters) => True {
        required {
            /// File identifier of the sticker
            pub sticker: String [into],
            /// New sticker position in the set, zero-based
            pub position: u32,
        }
    }
}
