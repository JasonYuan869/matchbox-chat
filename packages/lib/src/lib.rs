mod bootstrap;
mod state;

use matchbox_socket::WebRtcSocket;
use tracing::info;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn join_room(room_id: &str) {
    // This function will be called when the user joins a room.
    // You can use this to set up the UI or perform any other necessary actions.
    // For example, you might want to fetch the chat history for the room.
    let mut room = WebRtcSocket::new_reliable(room_id);

}