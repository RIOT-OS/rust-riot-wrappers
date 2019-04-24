use crate::gcoap::PacketBuffer;
use coap_message::{ReadableMessage, WritableMessage, Code, OptionNumber};

pub struct OptionsIterator<'a> {
    msg: &'a PacketBuffer,
    index: core::ops::Range<u16>,
}
impl<'a> Iterator for OptionsIterator<'a> {
    type Item = jnet::coap::Option<'a>;

    fn next(&mut self) -> Option<Self::Item> {

        struct FakeOption<'b> {
            number: u16,
            value: &'b [u8],
        }

        let i = self.index.next()?;
        let opt_num = self.msg.opt_number_by_index(i);
        let res = FakeOption { number: opt_num, value: self.msg.opt_by_index(i) };
        // FIXME add an abstraction that can actually be constructed
        let res: jnet::coap::Option<'a> = unsafe { core::mem::transmute(res) };
        Some(res)
    }
}

impl<'a> ReadableMessage<'a> for PacketBuffer {
    type OptionsIter = OptionsIterator<'a>;

    fn options(&'a self) -> Self::OptionsIter {
        OptionsIterator {
            msg: &self,
            index: 0..self.options_len(),
        }
    }

    fn get_code(&self) -> Code {
        let code = self.get_code_raw();
//         Code(code)

        // FIXME: not transparent repr, but doing it anyway for the brief period until this all
        // gets refactored to not depend on jnet types
        let result: Code = unsafe { core::mem::transmute(code) };
        assert!(result.class() << 5 | result.detail() == code);
        result
    }

    fn payload(&self) -> &[u8] { unimplemented!() }
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

impl<'a> WritableMessage for ResponseMessage<'a> {
    fn available_space(&self) -> usize {
        self.message.payload().len()
    }

    fn set_code<C: Into<Code>>(&mut self, code: C) {
        let code = code.into();
        self.message.set_code_raw(code.class() << 5 | code.detail());
    }

    fn add_option(&mut self, number: OptionNumber, value: &[u8]) {
        if self.payload_written.is_some() {
            panic!("Options can not be added after payload was added");
        }
        self.message.opt_add_opaque(number.into(), value).expect("Options exceed allocated buffer");
    }

    fn set_payload(&mut self, data: &[u8]) {
        self.payload_mut()[..data.len()].copy_from_slice(data);
        self.truncate(data.len());
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
        F: FnMut(OptionNumber, &mut [u8])
    {
        let len = self.message.options_len();
        for i in 0..len {
            let opt_num = self.message.opt_number_by_index(i);
            callback(opt_num.into(), self.message.opt_by_index_mut(i));
        }
    }
}
