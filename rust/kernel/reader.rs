// SPDX-License-Identifier: GPL-2.0

//! Traits for types that can be read from.

use kernel::error::Result;

/// This trait is based on the userspace BufRead trait, but it is different in that it does require
/// that the type implement Read.
///
/// This is useful in the kernel since there are many sources of data that can be read from where
/// the source provides data in chunks bigger than 1 byte at a time. For such types, BufRead is a
/// more primitive trait than Read, and Read can be implemented on top of BufRead.
pub trait BufRead {
    /// This method results in the type getting more data and storing it in a buffer.
    fn fill_buf(&mut self) -> Result<&[u8]>;

    /// When the consumer of this BufRead type has read `amt` bytes from the buffer provided by
    /// fill_buf(), they must call consume() so that the reader does not return those `amt` bytes
    /// again.
    fn consume(&mut self, amt: usize);
}

/// This trait is for types that can be read from by multiple readers. Each reader will get its own
/// BufReader to store that reader's state. For example, when a BufRead type is used to back a
/// file, calling open() will result in into_bufreader() providing a BufRead type that is read from
/// in read().
pub trait IntoBufReader {
    /// The type of BufRead that will be produced.
    type BufReader: BufRead;

    /// This method will be called to produce a BufRead type for self, allowing self to be read
    /// from.
    fn into_bufreader(self) -> Self::BufReader;
}
