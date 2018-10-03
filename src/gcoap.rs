use riot_sys::libc::{c_uint, c_void, CStr};
use riot_sys::{
    coap_get_blockopt,
    coap_hdr_t,
    coap_opt_add_uint,
    coap_opt_finish,
    coap_pkt_t,
    coap_resource_t,
    gcoap_finish,
    gcoap_listener_t,
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
pub struct PacketBuffer {
    pkt: *mut coap_pkt_t,
    buf: *mut u8,
    len: usize,
}

impl PacketBuffer {
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

            let headroom = coap_get_total_hdr_len(&*self.pkt) + 25 /* used to be 8 */;
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
where
    R: Send + FnMut(&mut PacketBuffer) -> isize,
{
    pub fn new(path: &'a CStr, methods: u32, callback: R) -> Self {
        Resource {
            callback,
            _path: path,
            resources: [coap_resource_t {
                path: path.as_ptr(),
                methods: methods,
                handler: None,
                context: 0 as *mut _,
            }],
            listener: gcoap_listener_t {
                resources: 0 as *const _,
                resources_len: 1,
                next: 0 as *mut _,
            },
            registered: false,
        }
    }

    unsafe extern "C" fn call_handler(
        pkt: *mut coap_pkt_t,
        buf: *mut u8,
        len: usize,
        context: *mut c_void,
    ) -> isize {
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

/// Resources can not be dropped, for they can not be unregistered from gcoap (there is no
/// gcoap_unregister_listener function).
impl<'a, R> Drop for Resource<'a, R> {
    fn drop(&mut self) {
        if self.registered {
            panic!("Registered resources must never be dropped")
        }
    }
}
