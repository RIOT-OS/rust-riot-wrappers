use riot_sys::libc::{
    CStr,
    c_void
};
use riot_sys::{
    coap_pkt_t,
    coap_resource_t,
    gcoap_listener_t,
    gcoap_register_listener,
    COAP_GET,
    COAP_FORMAT_NONE,
    gcoap_resp_init,
    gcoap_finish,
};

pub const GET: u32 = COAP_GET;

pub struct PayloadWriter<'a> {
    data: &'a mut [u8],
    cursor: usize,
}

impl<'a> ::core::fmt::Write for PayloadWriter<'a> {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        let mut s = s.as_bytes();
        let mut result = Ok(());
        if self.cursor + s.len() > self.data.len() {
            s = &s[..self.data.len() - self.cursor];
            result = Err(::core::fmt::Error);
        }
        self.data[self.cursor..self.cursor + s.len()].clone_from_slice(s);
        self.cursor += s.len();
        result
    }
}

/// A representation of the incoming or outgoing data on the server side of a request. This
/// includes the coap_pkt_t pre-parsed header and option pointers as well as the memory area
/// dedicated to returning the packet.
pub struct PacketBuffer {
    pkt: *mut coap_pkt_t,
    buf: *mut u8,
    len: usize,
}

impl PacketBuffer {
    pub fn response(&mut self, code: u32, format: u32, data_generator: impl FnOnce(&mut PayloadWriter)) -> isize {
        let init_success = unsafe { gcoap_resp_init(self.pkt, self.buf, self.len, code) };
        if init_success != 0 {
            return -1;
        }
        let buf = unsafe { ::core::slice::from_raw_parts_mut((*self.pkt).payload, (*self.pkt).payload_len as usize) };
        let mut writer = PayloadWriter { data: buf, cursor: 0 };
        data_generator(&mut writer);
        unsafe { gcoap_finish(self.pkt, writer.cursor, format) }
    }

    // FIXME I'd like to use the type system to ensure that a resposne can be used only once; would
    // be easy if I could have a owned PacketBuffer in the callback, but how can I then ensure that
    // it doesn't get moved to the outside? (Or do I -- expecting non-blocking responses?)
    pub fn response_empty(&mut self, code: u32) -> isize {
        // This is copied from the static gcoap_response implementation
        unsafe {
            let init_success = gcoap_resp_init(self.pkt, self.buf, self.len, code);
            if init_success != 0 {
                return -1;
            }
            gcoap_finish(self.pkt, 0, COAP_FORMAT_NONE)
        }
    }
}

/// A single registerable resource. It wraps the two distinct concepts of a gcoap_listener and a
/// gcoap_resource into a single entity, thus avoiding the issues with a LIMIT present in the shell
/// module as well as the need to sort the resources by path, at the expense of being a wasteful
/// linked list.
pub struct Resource<'a, R> {
    callback: R,
    // This is redundant with the pointer stored in the listener, but correctly captures its
    // lifetime.
    _path: &'a CStr,
    resources: [coap_resource_t; 1],
    listener: gcoap_listener_t,
    registered: bool, // 
}

impl<'a, R> Resource<'a, R>
    // R must be Send because it'll be executed in the gcoap thread
    where R: Send + FnMut(&mut PacketBuffer) -> isize
{
    pub fn new(path: &'a CStr, methods: u32, callback: R) -> Self {
        Resource {
            callback,
            _path: path,
            resources: [ coap_resource_t {
                path: path.as_ptr(),
                methods: methods,
                handler: None,
                context: 0 as *mut _
            }],
            listener: gcoap_listener_t {
                resources: 0 as *const _,
                resources_len: 1,
                next: 0 as *mut _,
            },
            registered: false,
        }
    }

    unsafe extern "C" fn call_handler(pkt: *mut coap_pkt_t, buf: *mut u8, len: usize, context: *mut c_void) -> isize {
        let s = context as *mut Resource<'a, R>;
        let s = &mut *s;
        let cb = &mut s.callback;
        let mut pb = PacketBuffer { pkt, buf, len };
        cb(&mut pb)
    }

    // FIXME: Make sure this stays pinned while registered.
    pub fn register(&mut self) {
        // Set up all the internal links to make the listener valid
        self.resources[0].handler = Some(Self::call_handler);
        self.resources[0].context = self as *mut _ as *mut c_void;
        self.listener.resources = self.resources.as_ptr();

        unsafe { gcoap_register_listener(&mut self.listener) };

        self.registered = true;
    }
}

impl<'a, R> Drop for Resource<'a, R> {
    fn drop(&mut self) {
        if self.registered {
            panic!("Regsitered resources mus tnever be dropped, there is no gcoap_unregister_listener.")
        }
    }
}
