
#[allow(dead_code)]

// this is the max frame size for 802-15-4 packets
const MAX_PACKET: usize = 127;

pub trait PacketKind {
    fn max(self) -> usize;
    fn min(self) -> usize;
    fn is_valid(self, len: usize) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BasePacketKind {
    Base,
    End,
    Any,
}

impl PacketKind for BasePacketKind {
    fn max(self) -> usize {
        match self {
            BasePacketKind::Any => MAX_PACKET,
            BasePacketKind::Base => MAX_PACKET,
            BasePacketKind::End => 0,
        }
    }

    fn min(self) -> usize {
        match self {
            BasePacketKind::Any => 1,
            BasePacketKind::Base => 0,
            BasePacketKind::End => 0,
        }
    }

    fn is_valid(self, len: usize) -> bool {
        (self.min()..self.max()).contains(&len)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ZigBeePacketKind {
    MacFrame,
    MacHeader,
    MacHeaderExt,
    MacPayload,
    MacCommand,
    MacData,
    MacBeacon,

    NwkFrame,
    NwkHeader,
    NwkHeaderExt,
    NwkPayload,
    NwkCommand,
    NwkData,

    ApsFrame,
    ApsHeader,
    ApsHeaderExt,
    ApsPayload,
    ApsCommand,
    ApsData,
    ApsRelay,

    ZdoFrame,
    ZclFrame,

    TlvPayload,
    Mic
}

// TODO this should be parameterized to allow users to implement their own protocols
#[derive(Clone, Copy, Debug)]
pub enum ProtocolPacketKind {
    Base(BasePacketKind),
}

#[allow(irrefutable_let_patterns)]
impl PartialEq<BasePacketKind> for ProtocolPacketKind {
    fn eq(&self, other: &BasePacketKind) -> bool {
        if let Self::Base(kind) = self {
            kind == other
        } else {
            false
        }
    }
}

mod tests {

}
