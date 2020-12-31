mod packet_protocol;
mod packet_handle;
mod packet_view;

pub use packet_handle::PacketHandle;
pub use packet_view::PacketView;
pub use packet_protocol::ProtocolPacketKind;

#[allow(dead_code)]
pub enum PacketError {
    BadArgument,
    MemoryError,
    InvalidDirection,
}

