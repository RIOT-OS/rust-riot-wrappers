//! Components acting on the netif pktsnip layer

// FIXME: Move some of mod.rs in here

use crate::gnrc_pktbuf::{Mode, NotEnoughSpace, Pktsnip, Writable};
use crate::thread::KernelPID;
use riot_sys::{gnrc_netif_hdr_t, gnrc_nettype_t_GNRC_NETTYPE_NETIF as GNRC_NETTYPE_NETIF};

/// A transparent wrapper around ``gnrc_netif_hdr_t`` that provides idiomatically typed fields
#[repr(transparent)] // gnrc_pktbuf ensures the right alignment through its _align function
#[doc(alias = "gnrc_netif_hdr_t")]
#[derive(Copy, Clone)]
pub struct Header(gnrc_netif_hdr_t);

impl Header {
    pub fn if_pid(&self) -> Option<KernelPID> {
        KernelPID::new(self.0.if_pid)
    }

    pub fn set_if_pid(&mut self, pid: KernelPID) {
        self.0.if_pid = pid.0;
    }
}

impl<M: Mode> Pktsnip<M> {
    /// Get the Netif header of the snip, if there is any thusly typed snip present
    // Note that we can *not* just implement this with &mut on a Writable Pktsnip, because
    // writability is only ever about the first snip
    pub fn netif_get_header(&self) -> Option<&Header> {
        let netif_snip = self.search_type(GNRC_NETTYPE_NETIF)?;
        debug_assert!(netif_snip.data.len() == core::mem::size_of::<gnrc_netif_hdr_t>());
        // unsafe: Header is a transparent wrapper around the actual gnrc_netif_hdr_t, and the
        // gnrc_netif_hdr_t itself is valid as per Pktsnip reqirements
        unsafe { (netif_snip.data.as_ptr() as *const Header).as_ref() }
    }

    /// Build a netif header around the Pktsnip
    #[doc(alias = "gnrc_netif_hdr_build")]
    pub fn netif_hdr_build_with(
        self,
        f: impl FnOnce(&mut Header),
    ) -> Result<Pktsnip<Writable>, NotEnoughSpace> {
        // It's a bit inefficient that we set it to 0 first and then let methods on the
        //
        // Beware that compared to other header builders, this *doesn't* immediately take the old
        // header, and we have to add it later
        let snip =
            unsafe { riot_sys::gnrc_netif_hdr_build(core::ptr::null(), 0, core::ptr::null(), 0) };
        if snip == 0 as *mut _ {
            Err(NotEnoughSpace)
        } else {
            // unsafe: snip is initialized, and to_ptr takes care of the refcounts
            unsafe { (*snip).next = self.to_ptr() };
            // unsafe: As with netif_get_header, plus we just created this for exclusive use
            let hdr = unsafe { ((*snip).data as *mut Header).as_mut() }
                .expect("Header was created just before");
            f(hdr);
            Ok(unsafe { Pktsnip::<Writable>::from_ptr(snip) })
        }
    }
}
