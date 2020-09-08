/// This module implements coap_message::ReadableMessage for, and a wrapper that provides
/// coap_message::WritableMessage around RIOT's coap_pkt_t.

use crate::gcoap::{PacketBuffer, PacketBufferOptIter, PacketBufferOptIterMut};
use coap_message::{ReadableMessage, MinimalWritableMessage, MutableWritableMessage, WithSortedOptions};

pub struct MessageOption<'a> {
    number: u16,
    value: &'a [u8],
}

impl<'a> coap_message::MessageOption for MessageOption<'a> {
    fn number(&self) -> u16 {
        self.number
    }

    fn value(&self) -> &[u8] {
        self.value
    }
}

pub struct OptionsIterator<'a>(PacketBufferOptIter<'a>);
impl<'a> Iterator for OptionsIterator<'a> {
    type Item = MessageOption<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (opt_num, slice) = self.0.next()?;
        Some(MessageOption { number: opt_num, value: slice })
    }
}

impl<'a> WithSortedOptions<'a> for PacketBuffer {
    // valid because gcoap just reads options from the message where they are stored in sequence
}

impl<'a> ReadableMessage<'a> for PacketBuffer {
    type Code = u8;
    type OptionsIter = OptionsIterator<'a>;
    type MessageOption = MessageOption<'a>;

    fn code(&self) -> Self::Code {
        self.get_code_raw()
    }

    fn payload(&self) -> &[u8] {
        self.payload()
    }

    fn options(&'a self) -> Self::OptionsIter {
        OptionsIterator(self.opt_iter())
    }
}

pub struct ResponseMessage<'a> {
    message: &'a mut PacketBuffer,
    payload_written: Option<usize>,
}

impl<'a> ResponseMessage<'a> {
    pub fn new(buf: &'a mut PacketBuffer) -> Self {
        // Can't really err; FIXME ensure that such a check won't affect ROM too much
        buf.resp_init(5 << 5).unwrap();

        ResponseMessage {
            message: buf,
            payload_written: None,
        }
    }

    pub fn finish(&self) -> isize {
        self.message.get_length(match self.payload_written {
            None => 0,
            Some(x) => x + 1,
        }) as isize
    }
}

impl<'a> MinimalWritableMessage for ResponseMessage<'a> {
    type Code = u8;
    type OptionNumber= u16;

    fn set_code(&mut self, code: Self::Code) {
        self.message.set_code_raw(code);
    }

    fn add_option(&mut self, number: Self::OptionNumber, value: &[u8]) {
        if self.payload_written.is_some() {
            panic!("Options can not be added after payload was added");
        }
        self.message.opt_add_opaque(number.into(), value).expect("Options exceed allocated buffer");
    }

    fn set_payload(&mut self, data: &[u8]) {
        self.payload_mut()[..data.len()].copy_from_slice(data);
        self.truncate(data.len());
    }
}

impl<'a> MutableWritableMessage for ResponseMessage<'a> {
    fn available_space(&self) -> usize {
        self.message.payload().len()
    }


    fn payload_mut(&mut self) -> &mut [u8] {
        self.payload_written = Some(0);
        let payload = self.message.payload_mut();
        payload[0] = 0xff;
        &mut payload[1..]
    }

    fn truncate(&mut self, len: usize) {
        self.payload_written = Some(len);
    }

    fn mutate_options<F>(&mut self, mut callback: F)
    where
        F: FnMut(Self::OptionNumber, &mut [u8])
    {
        for (opt_num, slice) in self.message.opt_iter_mut() {
            callback(opt_num.into(), slice);
        }
    }
}
