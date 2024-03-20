use coap_message_0_2::{
    MessageOption, MinimalWritableMessage, MutableWritableMessage, ReadableMessage,
    WithSortedOptions,
};

impl<'a> MessageOption for super::MessageOption<'a> {
    fn number(&self) -> u16 {
        self.number
    }

    fn value(&self) -> &[u8] {
        self.value
    }
}

impl WithSortedOptions for super::PacketBuffer {
    // valid because gcoap just reads options from the message where they are stored in sequence
}

impl ReadableMessage for super::PacketBuffer {
    type Code = u8;
    type OptionsIter<'a> = super::OptionsIterator<'a>;
    type MessageOption<'a> = super::MessageOption<'a>;

    fn code(&self) -> Self::Code {
        self.get_code_raw()
    }

    fn payload(&self) -> &[u8] {
        self.payload()
    }

    fn options(&self) -> Self::OptionsIter<'_> {
        super::OptionsIterator(self.opt_iter())
    }
}

impl<'a> MinimalWritableMessage for super::ResponseMessage<'a> {
    type Code = u8;
    type OptionNumber = u16;

    fn set_code(&mut self, code: Self::Code) {
        self.message.set_code_raw(code);
    }

    fn add_option(&mut self, number: Self::OptionNumber, value: &[u8]) {
        if self.payload_written.is_some() {
            panic!("Options can not be added after payload was added");
        }
        self.message
            .opt_add_opaque(number.into(), value)
            .expect("Options exceed allocated buffer");
    }

    fn set_payload(&mut self, data: &[u8]) {
        self.payload_mut_with_len(data.len()).copy_from_slice(data);
        self.truncate(data.len());
    }
}

impl<'a> MutableWritableMessage for super::ResponseMessage<'a> {
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
        F: FnMut(Self::OptionNumber, &mut [u8]),
    {
        for (opt_num, slice) in self.message.opt_iter_mut() {
            callback(opt_num.into(), slice);
        }
    }
}
