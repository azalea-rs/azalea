use azalea_client::chat::{ChatKind, SendChatEvent, handler::SendChatKindEvent};

use crate::Client;

impl Client {
    /// Send a chat message to the server.
    ///
    /// This only sends the chat packet and not the command packet, which means
    /// on some servers you can use this to send chat messages that start
    /// with a `/`. The [`Client::chat`] function handles checking whether
    /// the message is a command and using the proper packet for you, so you
    /// should use that instead.
    pub fn write_chat_packet(&self, message: &str) {
        self.ecs.write().write_message(SendChatKindEvent {
            entity: self.entity,
            content: message.to_owned(),
            kind: ChatKind::Message,
        });
    }

    /// Send a command packet to the server. The `command` argument should not
    /// include the slash at the front.
    ///
    /// You can also just use [`Client::chat`] and start your message with a `/`
    /// to send a command.
    pub fn write_command_packet(&self, command: &str) {
        self.ecs.write().write_message(SendChatKindEvent {
            entity: self.entity,
            content: command.to_owned(),
            kind: ChatKind::Command,
        });
    }

    /// Send a message in chat.
    ///
    /// ```rust,no_run
    /// # use azalea::Client;
    /// # async fn example(bot: Client) -> anyhow::Result<()> {
    /// bot.chat("Hello, world!");
    /// # Ok(())
    /// # }
    /// ```
    pub fn chat(&self, content: impl Into<String>) {
        self.ecs.write().write_message(SendChatEvent {
            entity: self.entity,
            content: content.into(),
        });
    }
}
