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
}
