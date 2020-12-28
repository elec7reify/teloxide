// This file is auto generated by `cg` <https://github.com/teloxide/cg> (9a82143).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::True;

impl_payload! {
    /// Use this method to delete a sticker from a set created by the bot. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub DeleteStickerFromSet (DeleteStickerFromSetSetters) => True {
        required {
            /// File identifier of the sticker
            pub sticker: String [into],
        }
    }
}
