//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{DiceEmoji, Message, MessageId, Recipient, ReplyMarkup};

impl_payload! {
    /// Use this method to send an animated emoji that will display a random value. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendDice (SendDiceSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
        }
        optional {
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: i32,
            /// Emoji on which the dice throw animation is based. Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”. Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀” and “⚽”, and values 1-64 for “🎰”. Defaults to “🎲”
            pub emoji: DiceEmoji,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
            /// If the message is a reply, ID of the original message
            #[serde(serialize_with = "crate::types::serialize_reply_to_message_id")]
            pub reply_to_message_id: MessageId,
            /// Pass _True_, if the message should be sent even if the specified replied-to message is not found
            pub allow_sending_without_reply: bool,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
