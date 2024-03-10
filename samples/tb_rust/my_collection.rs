use core::fmt;
use kernel::error::Result;
use kernel::prelude::*;
use kernel::reader::*;

#[derive(Clone)]
pub struct MyObject {
    pub data: usize,
}

unsafe impl Sync for MyObject {}

impl IntoBufReader for MyObject {
    type BufReader = MyReader;

    fn into_bufreader(self) -> Self::BufReader {
        MyReader::new(self.data)
    }
}

impl fmt::Display for MyObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyObject: {}", self.data)
    }
}

pub struct MyReader {
    data: usize,
    representation: Vec<u8>,
    pos: usize,
}

impl MyReader {
    pub fn new(data: usize) -> Self {
        let representation = Vec::new();

        MyReader {
            data,
            representation,
            pos: 0,
        }
    }
}

impl BufRead for MyReader {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        if self.data > 0 {
            if self.pos >= self.representation.len() {
                let mut buf = Vec::new();
                buf.try_extend_from_slice("hello bufreader\n".as_bytes())?;

                self.pos = 0;
                self.representation = buf;
                self.data -= 1;
            }
            Ok(&self.representation[self.pos..])
        } else {
            if self.pos < self.representation.len() {
                Ok(&self.representation[self.pos..])
            } else {
                Ok(&[])
            }
        }
    }

    fn consume(&mut self, amt: usize) {
        self.pos += amt;
    }
}
