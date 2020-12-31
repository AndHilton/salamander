
#[allow(dead_code)]
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

#[cfg(test)]
mod tests {

}
