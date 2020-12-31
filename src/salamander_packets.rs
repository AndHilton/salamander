pub use packet_handle::PacketHandle;

#[allow(dead_code)]
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

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum BasePacketKind {
        Base,
        End,
        Any,
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

}

#[allow(dead_code)]
pub mod packet_view {

    pub use crate::salamander_packets::PacketError;

    use crate::salamander_packets::packet_protocol::ProtocolPacketKind;
    use crate::salamander_packets::packet_protocol::BasePacketKind;
    use std::rc::Rc;

    ///
    /// PacketViews represent the discrete segments that comprise a ZigBee Packet.
    /// They provide a way of inspecting a portion of the packet with metadata to
    /// identify the relevant protocol layer.  Optionally, we can link PacketViews
    /// together in order to traverse the packet header by header.
    ///
    #[derive(Clone)]
    pub struct PacketView {
        kind: ProtocolPacketKind,
        len: usize,
        done: bool,
        left: ViewLink,
        right: ViewLink,
        data: ViewData,
    }

    pub type ViewLink = Option<Rc<PacketView>>;

    pub type ViewData = Option<packet_data_sources::PacketViewSource>;

    impl PacketView {
        pub fn new_view(opt_kind: Option<ProtocolPacketKind>, len: usize) -> PacketView {
            let kind = opt_kind.unwrap_or(ProtocolPacketKind::Base(BasePacketKind::Any));
            PacketView {
                kind,
                len,
                done: false,
                left: None,
                right: None,
                data: None,
            }
        }
    }

    impl PacketView {

        pub fn source_data_from_vec(&mut self, data_vec: Vec<u8>) -> Option<PacketError> {
            if self.data.is_some() {
                panic!()
            } else {
                let vec_source = packet_data_sources::ViewSourceVec::take_data(0, data_vec);
                self.data = Some(vec_source)
            }
            None
        }

        pub fn set_left(&mut self, opt_link: ViewLink) -> Option<PacketError> {
            if let Some(link) = opt_link {
                self.left = Some(Rc::clone(&link))
            } else {
                self.left = None
            }
            None
        }

        pub fn set_right(&mut self, opt_link: ViewLink) -> Option<PacketError> {
            if let Some(link) = opt_link {
                self.right = Some(Rc::clone(&link))
            } else {
                self.right = None
            }
            None
        }
    }

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

    mod packet_data_sources {

        #[derive(Clone)]
        pub enum PacketViewSource {
            // SliceView(PacketSourceSlice),
            VecView(ViewSourceVec),
            BoxView(ViewSourceBox),
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
                }
            }
        }

        // trait for getting data
        pub trait PacketSource {
            type Source;
            // fn borrow_data(index: usize, data:Source) -> PacketViewSource;
            fn take_data<T>(index: usize, data: T) -> PacketViewSource;

            fn as_slice(&self) -> &[u8];
            fn index(&self) -> usize;
        }

        #[derive(Clone)]
        pub struct ViewSourceRef <'a> {
            index: usize,
            data: &'a[u8],
        }

        #[derive(Clone)]
        pub struct ViewSourceVec {
            index: usize,
            data: Vec<u8>,
        }

        impl ViewSourceVec {
            pub fn take_data(index: usize, data: Vec<u8>) -> PacketViewSource {
                let source = ViewSourceVec {
                    index,
                    data,
                };
                PacketViewSource::VecView(source)
            }
        }

        #[derive(Clone)]
        pub struct ViewSourceBox {
            index: usize,
            data: Box<[u8]>,
        }

    }
}


#[cfg(test)]
mod tests {

    use crate::salamander_packets::*;
    // use packet_handle::*;
    use packet_protocol::*;
    use packet_view::*;


    #[test]
    fn basic_view_from_vector() {
        let test_string = String::from("Testing");
        let len = test_string.len();
        let mut view = PacketView::new_view(None, len);
        assert_eq!(view.kind(), BasePacketKind::Any);
        assert_eq!(view.len(), len);
        let data_vec = Vec::from(String::clone(&test_string));
        view.source_data_from_vec(data_vec);
        assert!(view.data().is_some());
        let view_slice = view.data().unwrap_or_else(|| panic!());
        assert_eq!(view_slice, test_string.as_bytes());
    }

}
