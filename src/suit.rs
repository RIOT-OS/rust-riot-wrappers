#[cfg(riot_module_suit_transport_coap)]
pub mod coap {
    /// Start SUIT CoAP thread
    ///
    /// FIXME: Is this safe to call multiple times? Do we need to provide an exclusion bit that panics
    /// if this is run twice?
    pub fn run() {
        unsafe { riot_sys::suit_coap_run() }
    }

    /// Trigger the SUIT thread to fetch a manifest and execute any indicated updates from the given
    /// URI
    pub fn trigger(uri: &str) {
        unsafe { riot_sys::suit_coap_trigger(uri.as_ptr(), uri.len() as _) }
    }

    #[cfg(feature = "with_coap_handler")]
    /// A trigger resource that allows unauthenticated CoAP users to trigger the SUIT update from
    /// an arbitrary manifest location
    ///
    /// **Warning:** While this is safe from a firmware update perspective (because the manifest is
    /// signed), it creates a traffic amplification vector. A better alternative would be sending
    /// the manifest to the to-be-updated device. This is still provided for compatibility with the
    /// RIOT built-in `suit/notify` target.
    pub struct TriggerHandler;

    #[cfg(feature = "with_coap_handler")]
    impl coap_handler::Handler for TriggerHandler {
        type RequestData = u8;

        fn extract_request_data(&mut self, request: &impl coap_message::ReadableMessage) -> u8 {
            match request.code().into() {
                coap_numbers::code::POST | coap_numbers::code::PUT => {
                    // FIXME check options
                    core::str::from_utf8(request.payload())
                        .map(trigger)
                        .map(|()| coap_numbers::code::CHANGED)
                        .unwrap_or(coap_numbers::code::BAD_REQUEST)
                },
                _ => coap_numbers::code::METHOD_NOT_ALLOWED
            }
        }

        fn estimate_length(&mut self, _: &<Self as coap_handler::Handler>::RequestData) -> usize {
            1
        }

        fn build_response(&mut self, response: &mut impl coap_message::MutableWritableMessage, request: Self::RequestData) {
            use core::convert::TryInto;
            response.set_code(request.try_into().map_err(|_| ()).expect("Message can't even express basic response codes"));
            response.set_payload(b"");
        }
    }
}
