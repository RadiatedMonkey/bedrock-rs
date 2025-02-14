use crate::events::handle::PlayerHandle;

pub struct ChatSendEvent {
    message: String,
    sender: PlayerHandle,
    targets: Option<Vec<PlayerHandle>>,
}
