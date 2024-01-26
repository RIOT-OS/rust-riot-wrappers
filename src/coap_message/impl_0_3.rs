use coap_message_0_3::{
    error::RenderableOnMinimal,
    Code,
    MessageOption,
    MinimalWritableMessage,
    MutableWritableMessage,
    ReadableMessage,
    WithSortedOptions,
};

use crate::error::NumericError;

/// Thin wrapper around NumericError that can render and satisfies all conversion requirements
#[derive(Debug)]
pub struct Error(NumericError);

impl From<NumericError> for Error {
    fn from(e: NumericError) -> Error {
        Error(e)
    }
}

impl From<core::convert::Infallible> for Error {
    fn from(e: core::convert::Infallible) -> Error {
        match e {}
    }
}

impl RenderableOnMinimal for Error {
    type Error<IE: RenderableOnMinimal + core::fmt::Debug> = IE;
    fn render<M: MinimalWritableMessage>(self, msg: &mut M) -> Result<(), M::UnionError> {
        msg.set_code(Code::new(coap_numbers::code::INTERNAL_SERVER_ERROR)?);
        Ok(())
    }
}

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

    type AddOptionError = Error;
    type SetPayloadError = Error;
    type UnionError = Error;

    fn set_code(&mut self, code: Self::Code) {
        self.message.set_code_raw(code);
    }

    fn add_option(&mut self, number: Self::OptionNumber, value: &[u8]) -> Result<(), Error> {
        if self.payload_written.is_some() {
            return Err(NumericError::from_constant(riot_sys::EINVAL as _).into());
        }
        self.message.opt_add_opaque(number.into(), value)?;
        Ok(())
    }

    fn set_payload(&mut self, data: &[u8]) -> Result<(), Error> {
        self.payload_mut_with_len(data.len())?.copy_from_slice(data);
        Ok(())
    }
}

impl<'a> MutableWritableMessage for super::ResponseMessage<'a> {
    fn available_space(&self) -> usize {
        self.message.payload().len()
    }

    fn payload_mut_with_len(&mut self, len: usize) -> Result<&mut [u8], Error> {
        self.payload_written = Some(len);
        let payload = self.message.payload_mut();
        if let Some((pm, pl)) = payload.get_mut(..len + 1).and_then(<[u8]>::split_first_mut) {
            *pm = 0xff;
            Ok(pl)
        } else {
            Err(NumericError::from_constant(riot_sys::EINVAL as _).into())
        }
    }

    fn truncate(&mut self, len: usize) -> Result<(), Error> {
        if self.payload_written.is_none() {
            // payload() will not even return anything sensible yet
            return Err(NumericError::from_constant(riot_sys::EINVAL as _).into());
        }
        let pl_len = self.message.payload().len() - 1;
        if len > pl_len {
            return Err(NumericError::from_constant(riot_sys::EINVAL as _).into());
        }
        self.payload_written = Some(len);
        Ok(())
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
