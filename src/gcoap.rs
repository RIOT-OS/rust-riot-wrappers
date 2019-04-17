// There is some questionably scoped code in the lower half of this module (it made requirements on
// data staying in a place that was not justified from the type system). This is being changed.

use riot_sys::{coap_resource_t, gcoap_listener_t, coap_pkt_t};
use riot_sys::libc::{CStr, c_void};
use core::marker::PhantomData;

/// Give the caller a way of registering Gcoap handlers into the global Gcoap registry inside a
/// callback. When the callback terminates, the registered handlers are deregistered again,
/// theoretically allowing the registration of non-'static handlers.
///
/// As there is currently no way to unregister handlers, this function panics when the callback
/// terminates.
pub fn scope<F>(callback: F)
where
    F: FnOnce(&mut RegistrationScope)
{
    let mut r = RegistrationScope { _private: () };

    callback(&mut r);

    r.deregister_all();
}

// Could we allow users the creation of 'static RegistrationScopes? Like thread::spawn.
pub struct RegistrationScope {
    _private: ()
}

impl RegistrationScope {
    // FIXME: Generalize SingleHandlerListener::get_listener into a trait
    pub fn register<'scope, 'handler, P>(&'scope mut self, handler: &'handler mut P) where
        'handler: 'scope,
        P: 'handler + ListenerProvider
    {
        // Unsafe: Moving in a pointer to an internal structure to which we were given an exclusive
        // reference that outlives self -- and whoever can create a Self guarantees that
        // deregister_all() will be called before the end of this self's lifetime.
        unsafe { gcoap_register_listener(handler.get_listener() as *mut _) };
    }

    fn deregister_all(&mut self) {
        panic!("Registration callback returned, but Gcoap does not allow deregistration.");
    }
}

pub trait ListenerProvider {
    /// Provide an exclusive reference to the underlying gcoap listener. The function is marked
    /// unsafe as the returned value contains raw pointers that will later be dereferenced, and
    /// returning arbitrary pointers would make RegistratinScope::register() pass bad data on to C.
    unsafe fn get_listener<'a>(&'a mut self) -> &'a mut gcoap_listener_t;
}

pub struct SingleHandlerListener<'a, H> {
    _phantom: PhantomData<&'a H>,
    resource: coap_resource_t,
    listener: gcoap_listener_t,
}

/// A combination of the coap_resource_t and gcoap_listener_t structs with only a single resource
/// (Compared to many resources, this allows easier creation in Rust at the expense of larger
/// memory consumption and slower lookups in Gcoap).
///
/// A listener `l` can be hooked into the global Gcoap registry using `scope(|x| { x.register(l)
/// })`.
impl<'a, H> SingleHandlerListener<'a, H>
where
    H: 'a + Handler
{
    pub fn new(path: &'a CStr, methods: u32, handler: &'a mut H) -> Self
    {
        SingleHandlerListener {
            _phantom: PhantomData,
            resource: coap_resource_t {
                path: path.as_ptr(),
                handler: Some(Self::call_handler),
                methods: methods,
                context: handler as *mut _ as *mut c_void,
            },
            listener: gcoap_listener_t {
                resources: 0 as *const _,
                resources_len: 0,
                next: 0 as *mut _,
            }
        }
    }

    unsafe extern "C" fn call_handler(
        pkt: *mut coap_pkt_t,
        buf: *mut u8,
        len: usize,
        context: *mut c_void,
    ) -> isize {
        let h = context as *mut H;
        let h = &mut *h;
        let mut pb = PacketBuffer { pkt, buf, len };
        H::handle(h, &mut pb)
    }
}

impl<'a, H> ListenerProvider for SingleHandlerListener<'a, H>
where
    H: 'a + Handler
{
    unsafe fn get_listener(&mut self) -> &mut gcoap_listener_t {
        self.listener.resources = &self.resource;
        self.listener.resources_len = 1;
        self.listener.next = 0 as *mut _;

        &mut self.listener
    }

}

pub trait Handler {
    fn handle(&mut self, pkt: &mut PacketBuffer) -> isize;
}

/// A wrapper that implements gnrc::Handler for closures. This allows easy compact writing of
/// ad-hoc handlers.
pub struct ClosureHandler<H>(pub H) where
    H: FnMut(&mut PacketBuffer) -> isize
;
impl<H> Handler for ClosureHandler<H> where
    H: FnMut(&mut PacketBuffer) -> isize
{
    fn handle(&mut self, pkt: &mut PacketBuffer) -> isize {
        self.0(pkt)
    }
}


// Questionable code starts here

use riot_sys::libc::{c_uint};
use riot_sys::{
    coap_get_blockopt,
    coap_hdr_t,
    coap_opt_add_uint,
    coap_opt_add_opaque,
    coap_opt_finish,
    gcoap_finish,
    gcoap_register_listener,
    gcoap_resp_init,
    memmove,
    COAP_FORMAT_NONE,
    COAP_GET,
    COAP_OPT_BLOCK2,
    COAP_OPT_CONTENT_FORMAT,
    COAP_OPT_FINISH_NONE,
    COAP_OPT_FINISH_PAYLOAD,
    COAP_OPT_OBSERVE,
    COAP_TYPE_ACK,
    COAP_TYPE_CON,
};
const COAP_OPT_ETAG: u16 = 4;
const COAP_OPT_SIZE2: u16 = 28;
// Static functions re-implemented
fn coap_get_total_hdr_len(pkt: &coap_pkt_t) -> usize {
    ::core::mem::size_of::<coap_hdr_t>() + coap_get_token_len(pkt)
}
fn coap_get_token_len(pkt: &coap_pkt_t) -> usize {
    (unsafe { (*pkt.hdr).ver_t_tkl & 0xfu8 }) as usize
}

use crc::crc64;

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

pub struct BlockWriter<'a> {
    pub data: &'a mut [u8],
    pub cursor: isize,
    pub etag: u64,
}

impl<'a> BlockWriter<'a> {
    fn bytes_in_buffer(&self) -> Option<usize> {
        if self.cursor < 0 {
            None
        } else {
            Some(::core::cmp::min(self.cursor as usize, self.data.len()))
        }
    }

    fn did_overflow(&self) -> bool {
        self.cursor > self.data.len() as isize
    }
}

impl<'a> ::core::fmt::Write for BlockWriter<'a> {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        let mut s = s.as_bytes();

        self.etag = crc64::update(self.etag, &crc64::ECMA_TABLE, s);

        if self.cursor >= self.data.len() as isize {
            // Still counting up to give a reliable Size2
            self.cursor += s.len() as isize;
            return Ok(());
        }
        if self.cursor < -(s.len() as isize) {
            self.cursor += s.len() as isize;
            return Ok(());
        }
        if self.cursor < 0 {
            s = &s[(-self.cursor) as usize..];
            self.cursor = 0;
        }

        let mut s_to_copy = s;
        if self.cursor as usize + s.len() > self.data.len() {
            let copy_bytes = self.data.len() - self.cursor as usize;
            s_to_copy = &s[..copy_bytes]
        }

        self.data[self.cursor as usize..self.cursor as usize + s_to_copy.len()]
            .copy_from_slice(s_to_copy);

        self.cursor += s.len() as isize;
        Ok(())
    }
}

/// A representation of the incoming or outgoing data on the server side of a request. This
/// includes the coap_pkt_t pre-parsed header and option pointers as well as the memory area
/// dedicated to returning the packet.
///
/// This struct wraps the unsafety of the C API, but does not structurally ensure that valid CoAP
/// messages are created. (For example, it does not keep the user from adding options after the
/// payload marker). Use CoAP generalization for that.
#[derive(Debug)]
pub struct PacketBuffer {
    pkt: *mut coap_pkt_t,
    buf: *mut u8,
    len: usize,
}

// Helper for error handling
trait ZeroOk {
    fn convert(self) -> Result<(), ()>;
}

// Use num-traits to get rid of the duplication? Would check for .is_zero() for num::Zero types
impl ZeroOk for i32 {
    fn convert(self) -> Result<(), ()> {
        match self.into() {
            0 => Ok(()),
            _ => Err(()),
        }
    }
}

impl ZeroOk for isize {
    fn convert(self) -> Result<(), ()> {
        match self.into() {
            0 => Ok(()),
            _ => Err(()),
        }
    }
}

impl PacketBuffer {
    /// Wrapper for coap_get_code_raw
    pub fn get_code_raw(&self) -> u8 {
        // FIXME inlining static coap_get_code_raw
        unsafe { (*(*self.pkt).hdr).code }
    }

    /// Wrapper for coap_get_total_hdr_len
    fn get_total_hdr_len(&self) -> usize {
        unsafe { coap_get_total_hdr_len(&*self.pkt) }
    }

    /// Wrapper for gcoap_resp_init
    ///
    /// As it is used and wrapped here, this makes GCOAP_RESP_OPTIONS_BUF bytes unusable, but
    /// working around that would mean duplicating code. Just set GCOAP_RESP_OPTIONS_BUF to zero to
    /// keep the overhead low.
    pub fn resp_init(&mut self, code: u8) -> Result<(), ()> {
        unsafe { gcoap_resp_init(self.pkt, self.buf, self.len, code.into()) }.convert()
    }

    pub fn set_code_raw(&mut self, code: u8) {
        unsafe { (*(*self.pkt).hdr).code  = code };
    }

    /// Return the total number of bytes in the message, given that `payload_used` bytes were
    /// written at the payload pointer. Note that those bytes have to include the payload marker.
    ///
    /// This measures the distance between the payload pointer in the pkt and the start of the
    /// buffer. It is the header length after `prepare_response`, and grows as options are added.
    pub fn get_length(&self, payload_used: usize) -> usize {
        let own_length = unsafe { (*self.pkt).payload.offset_from(self.buf) };
        assert!(own_length >= 0);
        let total_length = own_length as usize + payload_used;
        assert!(total_length <= self.len);
        total_length
    }

    /// A view of the current message payload
    ///
    /// This is only the CoAP payload after opt_finish has been called; before, it is a view on the
    /// remaining buffer space after any options that have already been added.
    pub fn payload(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts((*self.pkt).payload, (*self.pkt).payload_len as usize)
        }
    }

    /// A mutable view of the current message payload
    ///
    /// See `payload`.
    pub fn payload_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut((*self.pkt).payload, (*self.pkt).payload_len as usize)
        }
    }

    /// Add an integer value as an option
    pub fn opt_add_uint(&mut self, optnum: u16, value: u32) -> Result<(), ()> {
        unsafe { coap_opt_add_uint(self.pkt, optnum, value) }.convert()
    }

    /// Add a binary value as an option
    pub fn opt_add_opaque(&mut self, optnum: u16, data: &[u8]) -> Result<(), ()> {
        unsafe { coap_opt_add_opaque(self.pkt, optnum, data.as_ptr(), data.len()) }.convert()
    }


    pub fn response(
        &mut self,
        code: u32,
        format: u32,
        data_generator: impl FnOnce(&mut PayloadWriter),
    ) -> isize {
        let init_success = unsafe { gcoap_resp_init(self.pkt, self.buf, self.len, code) };
        if init_success != 0 {
            return -1;
        }
        let buf = unsafe {
            ::core::slice::from_raw_parts_mut((*self.pkt).payload, (*self.pkt).payload_len as usize)
        };
        let mut writer = PayloadWriter {
            data: buf,
            cursor: 0,
        };
        data_generator(&mut writer);
        unsafe { gcoap_finish(self.pkt, writer.cursor, format) }
    }

    /// This is an experimental responder that operates just like response, but trims the output to
    /// a 64byte block (given the typical 128byte limit of gcoap) based on the requested block. An
    /// ETag is calculated from a checksum of the whole message, which is rendered again and again
    /// for each block.
    pub fn response_blockwise(
        &mut self,
        code: u32,
        format: u32,
        data_generator: impl FnOnce(&mut BlockWriter),
    ) -> isize {
        let mut blknum: u32 = 0;
        let mut szx: c_uint = 6;
        match unsafe { coap_get_blockopt(self.pkt, COAP_OPT_BLOCK2 as u16, &mut blknum, &mut szx) }
        {
            -1 => {
                szx = 6;
            }
            _ => (), // ignore more flag
        }

        // The rest of this code assumes we ran gcoap_resp_init, but it'd only give us 8 bytes of
        // option space which is not enough. We're now beyond using gcoap in here actually, but
        // still emulating what it does because the rest of this function expects us to.
        // let init_success = unsafe { gcoap_resp_init(self.pkt, self.buf, self.len, code) };
        // if init_success != 0 {
        //     return -1;
        // }
        unsafe {
            let hdr: &mut coap_hdr_t = &mut *(*self.pkt).hdr;
            if (hdr.ver_t_tkl & 0x30) >> 4 == COAP_TYPE_CON as u8 {
                hdr.ver_t_tkl = hdr.ver_t_tkl & 0xcf | ((COAP_TYPE_ACK as u8) << 4);
            }
            hdr.code = code as u8;

            let headroom = self.get_total_hdr_len() + 25 /* used to be 8 */;
            (*self.pkt).payload = self.buf.offset(headroom as isize);
            (*self.pkt).payload_len = self.len as u16 - headroom as u16;
        }

        let buf = unsafe {
            ::core::slice::from_raw_parts_mut((*self.pkt).payload, (*self.pkt).payload_len as usize)
        };

        // Decrease block size until we'll fit into what has been pre-allocated
        let mut blksize: usize;
        loop {
            blksize = 1 << (4 + szx);
            if blksize < buf.len() {
                break;
            }
            blknum *= 2;
            szx = szx
                .checked_sub(1)
                .expect("Buffer too small for smalles block size");
        }

        let mut writer = BlockWriter {
            data: &mut buf[..blksize],
            cursor: -(blksize as isize * blknum as isize),
            etag: 0,
        };
        data_generator(&mut writer);

        let bytes_to_send = match writer.bytes_in_buffer() {
            None => {
                unimplemented!("Respond with error");
            }
            Some(x) => x,
        };
        let total_remaining_bytes = writer.cursor;
        let more = writer.did_overflow();

        // Before calling any coap_opt_add_* functions, we'll have to rewind the payload pointer so
        // that the functions know where to write (gcoap_finish does that implicitly by having a
        // buf there and not using gcoap_opt_add functions). The writer can happily live on because
        // the payload_len limitation tells nanocoap not to write into it.
        let bytes_to_send_start = unsafe { (*self.pkt).payload };
        unsafe {
            (*self.pkt).payload = self.buf.offset(coap_get_total_hdr_len(&*self.pkt) as isize);
            (*self.pkt).payload_len = bytes_to_send_start.offset_from((*self.pkt).payload) as u16;
            (*self.pkt).options_len = 0;
        }

        let etag = writer.etag;
        let etag = etag as u32; // FIXME: stripping to 4 bytes b/c there's no coap_opt_add_bytes option
        unsafe { coap_opt_add_uint(self.pkt, COAP_OPT_ETAG, etag) };

        // from gcoap_finish, see below
        let obsval = unsafe { (*self.pkt).observe_value };
        let has_observe = blknum == 0 && obsval != u32::max_value(); // inlining static coap_has_observe
        if has_observe {
            unsafe { coap_opt_add_uint(self.pkt, COAP_OPT_OBSERVE as u16, obsval) };
        }
        unsafe { coap_opt_add_uint(self.pkt, COAP_OPT_CONTENT_FORMAT as u16, format) };
        // end from gcoap_finish

        let block2 = (blknum << 4) | ((more as u32) << 3) | szx;
        unsafe { coap_opt_add_uint(self.pkt, COAP_OPT_BLOCK2 as u16, block2) };

        if blknum == 0 {
            // As we're in Block 0, the cursor gives the full length already and does not need the
            // block offset added.
            unsafe {
                coap_opt_add_uint(
                    self.pkt,
                    COAP_OPT_SIZE2 as u16,
                    total_remaining_bytes as u32,
                )
            };
        }

        // Not calling gcvoap_finish as that'd overwrite all options we just set.
        // Note that some code was later moved up to keep the sequence
        // unsafe { gcoap_finish(self.pkt, bytes_to_send, format) }

        // set observe, set content format: moved to top

        // Like the other coap_opt functions, this contains assertions that it does not overflow.
        unsafe {
            coap_opt_finish(
                self.pkt,
                if bytes_to_send != 0 {
                    COAP_OPT_FINISH_PAYLOAD as u16
                } else {
                    COAP_OPT_FINISH_NONE as u16
                },
            )
        };

        // They might easily alias.
        unsafe {
            memmove(
                (*self.pkt).payload as *mut _,
                bytes_to_send_start as *mut _,
                bytes_to_send,
            )
        };

        // FIXME this is bad pointer arithmetic
        (unsafe { (*self.pkt).payload } as usize + bytes_to_send - self.buf as usize) as isize
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
