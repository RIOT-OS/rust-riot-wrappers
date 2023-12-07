//! Wrapper containing functions from nanocoap_sock.h
//!

///Enum containing the possible error-codes of NanocoapClient
///
pub enum NanocoapClientErrors {
    /// The CoAP socket couldn't be created
    CreateSockErr,
    /// An unexpected error occurred
    UnexpectedErr,
    /// The CoAP request failed
    RequestErr,
    /// Couldn't fetch the content for the given URL
    FailedToFetchUrlContent,
    /// An invalid URL was provided
    InvalidURL,
    /// The provided buffer is too small
    BufferSizeNotSufficient,
    /// An invalid argument was provided
    InvalidArgument,
}

/// Used to create an empty coap socket
/// The fields are empty at first but get filled as the socket gets used
///
/// # Return
///
/// A newly created, empty coap socket gets returned
///
fn create_empty_nanocoap_sock_t() -> riot_sys::nanocoap_sock_t {
    unsafe {
        #[cfg(not(riot_module_nanocoap_dtls))]
        return riot_sys::nanocoap_sock_t {
            udp: core::mem::MaybeUninit::uninit().assume_init(),
            msg_id: core::mem::MaybeUninit::uninit().assume_init(),
        };
        #[cfg(riot_module_nanocoap_dtls)]
        return riot_sys::nanocoap_sock_t {
            udp: core::mem::MaybeUninit::uninit().assume_init(),
            msg_id: core::mem::MaybeUninit::uninit().assume_init(),
            dtls: core::mem::MaybeUninit::uninit().assume_init(),
            dtls_session: core::mem::MaybeUninit::uninit().assume_init(),
            type_: core::mem::MaybeUninit::uninit().assume_init(),
        };
    }
}

/// Used to create an empty udp socket
/// The fields are empty at first but get filled as the socket gets used
///
/// # Return
///
/// A newly created, empty udp socket gets returned
///
fn create_empty_udp_sock() -> riot_sys::sock_udp_t {
    unsafe {
        riot_sys::sock_udp_t {
            flags: core::mem::MaybeUninit::uninit().assume_init(),
            local: core::mem::MaybeUninit::uninit().assume_init(),
            reg: core::mem::MaybeUninit::uninit().assume_init(),
            remote: core::mem::MaybeUninit::uninit().assume_init(),
        }
    }
}

/// Simple synchronous CoAP (confirmable) PUT to URL
///
/// # Arguments
///
/// * `url` - Absolute URL to source path
/// * `request` - Buffer containing the payload
/// * `response` - Buffer for the response, may be None
///
/// # Return
///
/// Returns Ok with the length of the payload on success and Err with an error message otherwise
///
pub fn nanocoap_sock_put_url<'a>(
    url: &'a core::ffi::CStr,
    request: &[u8],
    response: Option<&mut [u8]>,
) -> Result<usize, NanocoapClientErrors> {
    let raw_request = request.as_ptr() as *const core::ffi::c_void;
    let raw_response;
    let response_len;
    match response {
        Some(response_buf) => {
            raw_response = response_buf.as_ptr() as *mut core::ffi::c_void;
            response_len = response_buf.len();
        }
        None => {
            raw_response = core::ptr::null_mut();
            response_len = 0;
        }
    }
    unsafe {
        let res = riot_sys::nanocoap_sock_put_url(
            url.as_ptr() as _,
            raw_request,
            request.len() as riot_sys::size_t,
            raw_response,
            response_len as riot_sys::size_t,
        );
        if res < 0 {
            Err(NanocoapClientErrors::RequestErr)
        } else {
            Ok(res as usize)
        }
    }
}

/// Simple synchronous CoAP (confirmable) POST to URL
///
/// # Arguments
///
/// * `url` - Absolute URL to source path
/// * `request` - Buffer containing the payload
/// * `response` - Buffer for the response, may be None
///
/// # Return
///
/// Returns Ok with the length of the payload on success and Err with an error message otherwise
///
pub fn nanocoap_sock_post_url<'a>(
    url: &'a core::ffi::CStr,
    request: &[u8],
    response: Option<&mut [u8]>,
) -> Result<usize, NanocoapClientErrors> {
    let raw_request = request.as_ptr() as *const core::ffi::c_void;
    let raw_response;
    let response_len;
    match response {
        Some(response_buf) => {
            raw_response = response_buf.as_ptr() as *mut core::ffi::c_void;
            response_len = response_buf.len();
        }
        None => {
            raw_response = core::ptr::null_mut();
            response_len = 0;
        }
    }
    unsafe {
        let res = riot_sys::nanocoap_sock_post_url(
            url.as_ptr() as _,
            raw_request,
            request.len() as riot_sys::size_t,
            raw_response,
            response_len as riot_sys::size_t,
        );
        if res < 0 {
            Err(NanocoapClientErrors::RequestErr)
        } else {
            Ok(res as usize)
        }
    }
}

/// Simple synchronous CoAP (confirmable) DELETE for URL
///
/// # Arguments
///
/// * `url` - Remote path to delete
///
/// # Return
///
/// Returns Ok on success and Err with an error message otherwise
///
pub fn nanocoap_sock_delete_url<'a>(url: &'a core::ffi::CStr) -> Result<(), NanocoapClientErrors> {
    unsafe {
        let res = riot_sys::nanocoap_sock_delete_url(url.as_ptr() as _);
        if res < 0 {
            Err(NanocoapClientErrors::RequestErr)
        } else {
            Ok(())
        }
    }
}

/// Struct used to store the relevant object used by a nanocoap client
///
pub struct NanocoapClient {
    sock: riot_sys::nanocoap_sock_t,
}

impl NanocoapClient {
    /// Creates a new NanocoapClient Object containing a nanocoap_sock_t object
    /// which is initialized with an empty udp socket
    ///
    /// # Arguments
    ///
    /// * `local` - The local endpoint, may be None
    /// * `remote` - The remote endpoint
    ///
    /// # Return
    ///
    /// Returns a new NanocoapClient object
    ///
    pub fn new(
        local: Option<&mut crate::socket::UdpEp>,
        remote: &mut crate::socket::UdpEp,
    ) -> Result<Self, NanocoapClientErrors> {
        let raw_local_ep: *mut riot_sys::sock_udp_ep_t;
        match local {
            Some(mut local_ep) => raw_local_ep = &mut local_ep.0 as &mut riot_sys::sock_udp_ep_t,
            None => raw_local_ep = core::ptr::null_mut(),
        }
        let raw_remote_ep: *mut riot_sys::sock_udp_ep_t =
            &mut remote.0 as &mut riot_sys::sock_udp_ep_t;
        let mut sock = create_empty_nanocoap_sock_t();
        let mut udp = create_empty_udp_sock();
        let raw_udp: *mut riot_sys::sock_udp_t = &mut udp as &mut riot_sys::sock_udp_t;
        unsafe {
            sock.udp = udp;
            //Created boundaries to prevent a overflow when casting to u16
            sock.msg_id = riot_sys::random_uint32_range(0, 65535) as u16;
            #[cfg(riot_module_nanocoap_dtls)]
            {
                sock.type_ = riot_sys::nanocoap_socket_type_t_COAP_SOCKET_TYPE_UDP;
            }
            let res = riot_sys::sock_udp_create(raw_udp, raw_local_ep, raw_remote_ep, 0);
            if res < 0 {
                Err(NanocoapClientErrors::CreateSockErr)
            } else {
                Ok(Self { sock })
            }
        }
    }

    #[cfg(riot_module_nanocoap_dtls)]
    /// Creates a new NanocoapClient Object with dtls containing a nanocoap_sock_t object
    /// which is initialized with an empty udp socket
    ///
    /// # Arguments
    ///
    /// * `local` - The local endpoint, may be None
    /// * `remote` - The remote endpoint
    /// * `tag` - Credential tag of sock. The sock will only use credentials with the tags
    ///           registered to it (see sock_dtls_add_credential).
    ///           Set to CREDMAN_TAG_EMPTY to create a sock with an empty tag list.
    ///
    /// # Return
    ///
    /// Returns a new NanocoapClient object
    ///
    pub fn new_dtls(
        local: Option<&mut crate::socket::UdpEp>,
        remote: &mut crate::socket::UdpEp,
        tag: riot_sys::credman_tag_t,
    ) -> Result<Self, NanocoapClientErrors> {
        let raw_local_ep: *mut riot_sys::sock_udp_ep_t;
        match local {
            Some(mut local_ep) => raw_local_ep = &mut local_ep.0 as &mut riot_sys::sock_udp_ep_t,
            None => raw_local_ep = core::ptr::null_mut(),
        }
        let raw_remote_ep: *mut riot_sys::sock_udp_ep_t =
            &mut remote.0 as &mut riot_sys::sock_udp_ep_t;
        let mut sock = create_empty_nanocoap_sock_t();
        let raw_sock: *mut riot_sys::nanocoap_sock_t = &mut sock as &mut riot_sys::nanocoap_sock_t;
        unsafe {
            let res =
                riot_sys::nanocoap_sock_dtls_connect(raw_sock, raw_local_ep, raw_remote_ep, tag);
            if res < 0 {
                return Err(NanocoapClientErrors::CreateSockErr);
            }
            Ok(Self { sock })
        }
    }

    /// Create a new NanocoapClient Object from a URL
    /// Throws an error when the creation of the socket fails
    ///
    /// # Arguments
    ///
    /// * `url` - URL with server information to connect to
    ///
    /// # Return
    ///
    /// Returns a new NanocoapClient object
    ///
    pub fn new_from_url<'a>(url: &'a core::ffi::CStr) -> Result<Self, NanocoapClientErrors> {
        let mut sock = create_empty_nanocoap_sock_t();
        let raw_sock: *mut riot_sys::nanocoap_sock_t = &mut sock as &mut riot_sys::nanocoap_sock_t;
        unsafe {
            let res = riot_sys::nanocoap_sock_url_connect(url.as_ptr() as _, raw_sock);
            if res < 0 {
                Err(NanocoapClientErrors::CreateSockErr)
            } else {
                Ok(Self { sock })
            }
        }
    }

    /// Get next consecutive message ID for use when building a new CoAP request.
    ///
    /// # Arguments
    ///
    /// * `self` - The object itself
    ///
    /// # Return
    ///
    /// A new message ID that can be used for a request or response.
    ///
    pub fn nanocoap_sock_next_msg_id(&mut self) -> u16 {
        self.sock.msg_id += 1;
        self.sock.msg_id
    }

    /// Simple synchronous CoAP (confirmable) GET
    ///
    /// # Arguments
    ///
    /// * `self` - The object itself
    /// * `url` - Remote path
    /// * `response` - Buffer to write response to
    ///
    /// # Return
    ///
    /// Returns Ok with the length of the payload on success and Err with an error message otherwise
    ///
    pub fn nanocoap_sock_get<'a>(
        &mut self,
        path: &'a core::ffi::CStr,
        response: &mut [u8],
    ) -> Result<usize, NanocoapClientErrors> {
        let raw_sock: *mut riot_sys::nanocoap_sock_t =
            &mut self.sock as &mut riot_sys::nanocoap_sock_t;
        let raw_response = response.as_ptr() as *mut core::ffi::c_void;
        unsafe {
            let res = riot_sys::nanocoap_sock_get(
                raw_sock,
                path.as_ptr() as _,
                raw_response,
                response.len() as riot_sys::size_t,
            );
            if res < 0 {
                Err(NanocoapClientErrors::RequestErr)
            } else {
                Ok(res as usize)
            }
        }
    }

    /// Simple synchronous CoAP (confirmable) PUT
    ///
    /// # Arguments
    ///
    /// * `self` - The object itself
    /// * `path` - Remote path
    /// * `request` - Buffer containing the payload
    /// * `response` - Buffer for the response, may be None
    ///
    /// # Return
    ///
    /// Returns Ok with the length of the payload on success and Err with an error message otherwise
    ///
    pub fn nanocoap_sock_put<'a>(
        &mut self,
        path: &'a core::ffi::CStr,
        request: &[u8],
        response: Option<&mut [u8]>,
    ) -> Result<usize, NanocoapClientErrors> {
        let raw_sock: *mut riot_sys::nanocoap_sock_t =
            &mut self.sock as &mut riot_sys::nanocoap_sock_t;
        let raw_request = request.as_ptr() as *const core::ffi::c_void;
        let raw_response;
        let response_len;
        match response {
            Some(response_buf) => {
                raw_response = response_buf.as_ptr() as *mut core::ffi::c_void;
                response_len = response_buf.len();
            }
            None => {
                raw_response = core::ptr::null_mut();
                response_len = 0;
            }
        }
        unsafe {
            let res = riot_sys::nanocoap_sock_put(
                raw_sock,
                path.as_ptr() as _,
                raw_request,
                request.len() as riot_sys::size_t,
                raw_response,
                response_len as riot_sys::size_t,
            );
            if res < 0 {
                Err(NanocoapClientErrors::RequestErr)
            } else {
                Ok(res as usize)
            }
        }
    }

    /// Simple non-confirmable PUT
    ///
    /// # Arguments
    ///
    /// * `self` - The object itself
    /// * `path` - Remote path
    /// * `request` - Buffer containing the payload
    /// * `response` - Buffer for the response, may be NULL
    ///
    /// # Return
    ///
    /// Returns Ok with the length of the payload on success and Err with an error message otherwise
    /// Returns Ok(0) when no response buffer is provided
    ///
    pub fn nanocoap_sock_put_non<'a>(
        &mut self,
        path: &'a core::ffi::CStr,
        request: &[u8],
        response: Option<&mut [u8]>,
    ) -> Result<usize, NanocoapClientErrors> {
        let raw_sock: *mut riot_sys::nanocoap_sock_t =
            &mut self.sock as &mut riot_sys::nanocoap_sock_t;
        let raw_request = request.as_ptr() as *const core::ffi::c_void;
        let raw_response;
        let response_len;
        match response {
            Some(response_buf) => {
                raw_response = response_buf.as_ptr() as *mut core::ffi::c_void;
                response_len = response_buf.len();
            }
            None => {
                raw_response = core::ptr::null_mut();
                response_len = 0;
            }
        }
        unsafe {
            let res = riot_sys::nanocoap_sock_put_non(
                raw_sock,
                path.as_ptr() as _,
                raw_request,
                request.len() as riot_sys::size_t,
                raw_response,
                response_len as riot_sys::size_t,
            );
            if res < 0 {
                Err(NanocoapClientErrors::RequestErr)
            } else {
                Ok(res as usize)
            }
        }
    }

    /// Simple synchronous CoAP (confirmable) POST
    ///
    /// # Arguments
    ///
    /// * `self` - The object itself
    /// * `path` - Remote path
    /// * `request` - Buffer containing the payload
    /// * `response` - Buffer for the response, may be NULL
    ///
    /// # Return
    ///
    /// Returns Ok with the length of the payload on success and Err with an error message otherwise
    ///
    pub fn nanocoap_sock_post<'a>(
        &mut self,
        path: &'a core::ffi::CStr,
        request: &[u8],
        response: Option<&mut [u8]>,
    ) -> Result<usize, NanocoapClientErrors> {
        let raw_sock: *mut riot_sys::nanocoap_sock_t =
            &mut self.sock as &mut riot_sys::nanocoap_sock_t;
        let raw_request = request.as_ptr() as *const core::ffi::c_void;
        let raw_response;
        let response_len;
        match response {
            Some(response_buf) => {
                raw_response = response_buf.as_ptr() as *mut core::ffi::c_void;
                response_len = response_buf.len();
            }
            None => {
                raw_response = core::ptr::null_mut();
                response_len = 0;
            }
        }
        unsafe {
            let res = riot_sys::nanocoap_sock_post(
                raw_sock,
                path.as_ptr() as _,
                raw_request,
                request.len() as riot_sys::size_t,
                raw_response,
                response_len as riot_sys::size_t,
            );
            if res < 0 {
                Err(NanocoapClientErrors::RequestErr)
            } else {
                Ok(res as usize)
            }
        }
    }


    /// Simple non-confirmable POST
    ///
    /// # Arguments
    ///
    /// * `self` - The object itself
    /// * `path` - Remote path
    /// * `request` - Buffer containing the payload
    /// * `response` - Buffer for the response, may be NULL
    ///
    /// # Return
    ///
    /// Returns Ok with the length of the payload on success and Err with an error message otherwise
    /// Returns Ok(0) when no response buffer is provided
    ///
    pub fn nanocoap_sock_post_non<'a>(
        &mut self,
        url: &'a core::ffi::CStr,
        request: &[u8],
        response: Option<&mut [u8]>,
    ) -> Result<usize, NanocoapClientErrors> {
        let raw_sock: *mut riot_sys::nanocoap_sock_t =
            &mut self.sock as &mut riot_sys::nanocoap_sock_t;
        let raw_request = request.as_ptr() as *const core::ffi::c_void;
        let raw_response;
        let response_len;
        match response {
            Some(response_buf) => {
                raw_response = response_buf.as_ptr() as *mut core::ffi::c_void;
                response_len = response_buf.len();
            }
            None => {
                raw_response = core::ptr::null_mut();
                response_len = 0;
            }
        }
        unsafe {
            let res = riot_sys::nanocoap_sock_post_non(
                raw_sock,
                url.as_ptr() as _,
                raw_request,
                request.len() as riot_sys::size_t,
                raw_response,
                response_len as riot_sys::size_t,
            );
            if res < 0 {
                Err(NanocoapClientErrors::RequestErr)
            } else {
                Ok(res as usize)
            }
        }
    }

    /// Simple synchronous CoAP (confirmable) DELETE
    ///
    /// # Arguments
    ///
    /// * `self` - The object itself
    /// * `path` - Remote path to delete
    ///
    /// # Return
    ///
    /// Returns Ok on success and Err with an error message otherwise
    ///
    pub fn nanocoap_sock_delete<'a>(
        &mut self,
        path: &'a core::ffi::CStr,
    ) -> Result<(), NanocoapClientErrors> {
        let raw_sock: *mut riot_sys::nanocoap_sock_t =
            &mut self.sock as &mut riot_sys::nanocoap_sock_t;
        unsafe {
            let res = riot_sys::nanocoap_sock_delete(raw_sock, path.as_ptr() as _);
            if res < 0 {
                Err(NanocoapClientErrors::RequestErr)
            } else {
                Ok(())
            }
        }
    }

    /// Simple synchronous CoAP request
    ///
    /// # Arguments
    ///
    /// * `self` - The object itself
    /// * `pkt` - Packet struct containing the request. Is reused for the response
    /// * `len` - Total length of the buffer associated with the request
    ///
    /// # Return
    ///
    /// Returns Ok with the length of the response on success and Err with an error message
    /// otherwise
    ///
    pub fn nanocoap_sock_request(
        &mut self,
        mut pkt: riot_sys::coap_pkt_t,
        len: usize,
    ) -> Result<usize, NanocoapClientErrors> {
        let raw_sock: *mut riot_sys::nanocoap_sock_t =
            &mut self.sock as &mut riot_sys::nanocoap_sock_t;
        let raw_pkt: *mut riot_sys::coap_pkt_t = &mut pkt as &mut riot_sys::coap_pkt_t;
        unsafe {
            let res = riot_sys::nanocoap_sock_request(raw_sock, raw_pkt, len as riot_sys::size_t);
            if res < 0 {
                Err(NanocoapClientErrors::RequestErr)
            } else {
                Ok(res as usize)
            }
        }
    }
}

/// Gets called when the object gets out of scope
/// Closes the udp socket used by the client
///
#[cfg(not(riot_module_nanocoap_dtls))]
impl Drop for NanocoapClient {
    fn drop(&mut self) {
        let raw_udp: *mut riot_sys::sock_udp_t = &mut self.sock.udp as &mut riot_sys::sock_udp_t;
        unsafe {
            riot_sys::sock_udp_close(raw_udp);
        }
    }
}

/// Gets called when the object gets out of scope
/// Closes the udp socket and dtls session used by the client
///
#[cfg(riot_module_nanocoap_dtls)]
impl Drop for NanocoapClient {
    fn drop(&mut self) {
        if self.sock.type_ == riot_sys::nanocoap_socket_type_t_COAP_SOCKET_TYPE_DTLS {
            let raw_dtls_sock: *mut riot_sys::sock_dtls_t =
                &mut self.sock.dtls as &mut riot_sys::sock_dtls_t;
            let raw_dtls_session: *mut riot_sys::sock_dtls_session =
                &mut self.sock.dtls_session as &mut riot_sys::sock_dtls_session;
            unsafe {
                riot_sys::sock_dtls_session_destroy(raw_dtls_sock, raw_dtls_session);
                riot_sys::sock_dtls_close(raw_dtls_sock);
            }
        }
        let raw_udp: *mut riot_sys::sock_udp_t = &mut self.sock.udp as &mut riot_sys::sock_udp_t;
        unsafe {
            riot_sys::sock_udp_close(raw_udp);
        }
    }
}

/// Enum containing the possible error-codes of NanocoapServer
///
pub enum NanocoapServerErrors {
    /// When binding to the local endpoint fails or packets are dropped
    BindingLocalOrDroppedPacketsErr,
}

/// Struct used to store relevant objects used by a nanocoap server
///
pub struct NanocoapServer {
    local: crate::socket::UdpEp,
}

impl NanocoapServer {
    /// Create a new NanocoapServer object
    ///
    /// # Arguments
    ///
    /// * `local` - The local endpoint
    ///
    /// # Return
    ///
    /// Returns a new NanocoapServer object
    ///
    pub fn new(local: crate::socket::UdpEp) -> Self {
        Self { local }
    }

    /// Start a nanocoap server instance
    /// Throws error if the binding to local fails or if the receiving of udp packets fail
    ///
    /// # Arguments
    ///
    /// * `self` - The object itself
    /// * `buf` - Input buffer to use
    ///
    /// # Return
    ///
    /// Returns Ok on success and a BindingLocalOrDroppedPacketsErr when the binding to local fails
    /// or if udp packets are dropped
    ///
    pub fn nanocoap_server(&mut self, mut buf: &[u8]) -> Result<(), NanocoapServerErrors> {
        let raw_local: *mut riot_sys::sock_udp_ep_t =
            &mut self.local.0 as &mut riot_sys::sock_udp_ep_t;
        let raw_buf = buf.as_ptr() as *mut u8;
        unsafe {
            let mut res = 0;
            //Function only returns if theres an error binding to local or if receiving UDP Packets fails
            res = riot_sys::nanocoap_server(raw_local, raw_buf, buf.len() as riot_sys::size_t);
            if res == -1 {
                Err(NanocoapServerErrors::BindingLocalOrDroppedPacketsErr)
            } else {
                Ok(())
            }
        }
    }
}
