pub use packet_handle::PacketHandle;

pub enum PacketError {
    BadArgument,
    MemoryError,
    InvalidDirection,
}

#[allow(dead_code)]
pub mod packet_handle {

    pub use crate::salamander_packets::PacketError;
    pub use crate::salamander_packets::packet_view::PacketView;

    pub struct PacketHandle {
        views: Vec<PacketView>,
    }

    impl PacketHandle {
        /// this function will create a right-bound (parsing) packet handle using the given data
        /// the packet will contain the base view, showing the whole of the packet
        pub fn from_source(source: &[u8]) -> PacketHandle {
            unimplemented!()
        }

        /// this function will create a left-bound (generating) packet handle that is empty
        pub fn empty_packet() -> PacketHandle {
            unimplemented!()
        }
    }

    impl PacketHandle {

        /// returns the base view of a given handle
        pub fn get_base(&self) -> PacketView {
            unimplemented!()
        }

        /// appends the given view to the left-bound chain
        pub fn push_left(&self, view: PacketView) -> Option<PacketError> {
            unimplemented!()
        }

        /// appends the given view to the right-bound chain
        pub fn push_right(&self, view: PacketView) ->Option<PacketError> {
            unimplemented!()
        }

        /// returns the view at the end of the left-bound chain
        pub fn pop_left(&self) -> Result<PacketView, PacketError> {
            unimplemented!()
        }

        /// returns the view at the end of the right-bound chain
        pub fn pop_right(&self) -> Result<PacketView, PacketError> {
            unimplemented!()
        }
    }
}

#[allow(dead_code)]
pub mod packet_protocol {

    // this is the max frame size for 802-15-4 packets
    const MAX_PACKET: usize = 127;

    pub trait PacketKind {
        fn max(self) -> usize;
        fn min(self) -> usize;
        fn is_valid(self, len: usize) -> bool;
    }

    #[derive(Clone, Copy)]
    pub enum BasePacketKind {
        Base,
        End,
        Any,
    }

    #[derive(Clone, Copy)]
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

    impl PacketKind for BasePacketKind {
        fn max(self) -> usize {
            match self {
                BasePacketKind::Base => MAX_PACKET,
                BasePacketKind::Any => MAX_PACKET,
                BasePacketKind::End => 0,
            }
        }

        fn min(self) -> usize {
            match self {
                BasePacketKind::Base => 1,
                BasePacketKind::Any => 1,
                BasePacketKind::End => 0,
            }
        }

        fn is_valid(self, len: usize) -> bool {
           (self.min()..self.max()).contains(&len)
        }
    }

    // TODO this should be parameterized to allow users to implement their own protocols
    #[derive(Clone, Copy)]
    pub enum ProtocolPacketKind {
        Base(BasePacketKind),
    }

}

#[allow(dead_code)]
pub mod packet_view {

    use crate::salamander_packets::packet_protocol::ProtocolPacketKind;

    ///
    /// PacketViews represent the discrete segments that comprise a ZigBee Packet.
    /// They provide a way of inspecting a portion of the packet with metadata to
    /// identify the relevant protocol layer.  Optionally, we can link PacketViews
    /// together in order to traverse the packet header by header.
    ///
    pub struct PacketView {
        kind: ProtocolPacketKind,
        len: usize,
        done: bool,
        left: ViewLink,
        right: ViewLink,
        data: ViewData,
    }

    type ViewLink = Option<Box<PacketView>>;

    impl PacketView {
        pub fn kind(&self) -> ProtocolPacketKind {
            self.kind
        }

        /// returns the PacketView data as a slice
        pub fn data(&self) -> Option<&[u8]> {
            if let Some(data) = &self.data {
                Some(data.as_slice(self.len()))
            } else {
                None
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn get_left(&self) -> Option<&PacketView> {
            // self.left
            if let Some(link) = &self.left {
                Some(&link)
            } else {
                None
            }
        }

        pub fn get_right(&self) -> Option<&PacketView> {
            // self.right
            if let Some(link) = &self.right {
                Some(&link)
            } else {
                None
            }
        }
    }

    pub type ViewData = Option<packet_data_sources::PacketViewSource>;

    mod packet_data_sources {
        pub enum PacketViewSource {
            // SliceView(PacketSourceSlice),
            VecView(PacketSourceVec),
            BoxView(PacketSourceBox),
        }

        impl PacketViewSource {
            pub fn as_slice(&self, len : usize) -> &[u8] {
                match self {
                    PacketViewSource::BoxView(data_box) => {
                        &data_box.data[data_box.index .. len ]
                    }
                    PacketViewSource::VecView(data_vec) => {
                        &data_vec.data[data_vec.index .. len ]
                    }
                    // _ => unimplemented!(),
                }
            }
        }

        // trait for getting data
        // pub trait PacketSource {
        //     fn as_slice(&self) -> &[u8];
        //     fn index(&self) -> usize;
        // }

        pub struct PacketSourceVec {
            index: usize,
            data: Vec<u8>,
        }

        pub struct PacketSourceBox {
            index: usize,
            data: Box<[u8]>,
        }

    }
}


// #[cfg(test)]
// mod tests {

// }
