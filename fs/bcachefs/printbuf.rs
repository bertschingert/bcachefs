//! Rust bindings for the C printbuf type

use crate::bindings;

/// A Printbuf wrapper
pub struct Printbuf {
    raw: bindings::printbuf,
}

impl Printbuf {
    /// Create a new Printbuf
    pub fn new() -> Printbuf {
        let mut raw: bindings::printbuf = Default::default();

        raw.set_heap_allocated(true);

        Printbuf { raw }
    }

    /// Get a reference to the inner printbuf. Used for calling C functions that take
    /// `struct printbuf *` as an argument.
    pub fn raw_mut(&mut self) -> &mut bindings::printbuf {
        &mut self.raw
    }

    /// Get the length of the data stored in the printbuf's buffer.
    pub fn len(&self) -> core::ffi::c_uint {
        self.raw.pos
    }

    /// Get the printbuf's buffer as a slice. TODO: make this unsafe?
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: todo...
        unsafe { core::slice::from_raw_parts(self.raw.buf as *const u8, self.raw.pos as usize) }
    }

    /// Clear the printbuf's buffer to make room for new data.
    ///
    /// Equivalent to printbuf_reset() in C
    pub fn reset(&mut self) {
        self.raw.pos = 0;
        self.raw.set_allocation_failure(false);
        self.raw.indent = 0;
        self.raw.nr_tabstops = 0;
        self.raw.cur_tabstop = 0;
    }

    /// Write a newline to the printbuf.
    pub fn newline(&mut self) {
        unsafe {
            bindings::bch2_prt_newline(self.raw_mut());
        }
    }
}

impl Drop for Printbuf {
    fn drop(&mut self) {
        // SAFETY: it is valid to drop a Printbuf because ...
        unsafe {
            bindings::bch2_printbuf_exit(&mut self.raw);
        }
    }
}
