use jnet;
use gnrc;

impl<M: gnrc::Mode> ::core::convert::AsRef<[u8]> for gnrc::Pktsnip<M> {
    fn as_ref(&self) -> &[u8] {
        self.get_data()
    }
}

impl ::core::convert::AsMut<[u8]> for gnrc::Pktsnip<gnrc::Writable> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.get_data_mut()
    }
}

impl<'a> jnet::Resize for &'a mut gnrc::Pktsnip<gnrc::Writable> {
    fn slice_from(&mut self, offset: u16) {
        // Not sure that's possible with Pktsnips
        unimplemented!();
    }

    fn truncate(&mut self, len: u16) {
        self.realloc_data(len as usize).unwrap();
    }
}
