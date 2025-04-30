use matchbox_socket::{RtcIceServerConfig, WebRtcSocket};

pub(crate) struct State {
    pub(crate) room_id: String,
    pub(crate) username: String,
    pub(crate) ice_config: RtcIceServerConfig,
    pub(crate) socket: WebRtcSocket,
}