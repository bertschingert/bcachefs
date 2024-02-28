use kernel::prelude::*;
use kernel::uaccess;
use kernel::str::CString;

use core::fmt;
use core::ffi;

struct MyCollection {
    num_objects: usize,
}

struct MyObject {
    data: usize,
}

impl fmt::Display for MyObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyObject: {}", self.data)
    }
}

impl Iterator for MyCollection {
    type Item = MyObject;

    fn next(&mut self) -> Option<Self::Item> {
        self.num_objects -= 1;

        if self.num_objects > 0 {
            Some (MyObject {
                data: self.num_objects
            })
        } else {
            None
        }
    }
}

pub struct DebugfsState {
    buf: Option<Box<CString>>,
    data: MyCollection,
}

impl DebugfsState {
    pub fn flush_buf(&mut self, len: usize, writer: &mut uaccess::UserSliceWriter) -> isize {
        let mut new_buf: Option<Box<CString>> = None;
        if let Some(buf) = &self.buf {
            let n = core::cmp::min(len, buf.len());

            if n > 0 {
                match writer.write_slice(&buf[0..n]) {
                    Ok(_) => {},
                    Err(_) => return -14,
                };

                if n < buf.len() {
                    let remainder = match CString::try_from(&buf[n..]) {
                        Ok(s) => s,
                        Err(_) => return -14,
                    };

                    new_buf = match Box::try_new(remainder) {
                        Ok(b)=> Some(b),
                        Err(_) => return -14,
                    };
                }

                self.buf = new_buf;

                return n.try_into().unwrap();
            }
        }

        0
    }
}

#[no_mangle]
pub extern "C" fn tb_rs_read(private_data: *mut DebugfsState,
                             uptr: *mut ffi::c_void,
                             len: usize,
                             off: *mut usize) -> isize {
    let state = unsafe { &mut *private_data };

    if state.buf.is_none() || state.buf.as_ref().unwrap().len() == 0 {
        let item = match state.data.next() {
            Some(i) => i,
            None => return 0,
        };

        let data_string = match CString::try_from_fmt(kernel::fmt!("tb_rs_read: data: {}\n", item)) {
            Ok(s) => s,
            Err(_) => return -14,
        };

        let data_string = match Box::try_new(data_string) {
            Ok(s) => s,
            Err(_) => return -14,
        };

        state.buf = Some(data_string);
    }

    let mut writer = uaccess::UserSlice::new(uptr, len).writer();

    let written = state.flush_buf(len, &mut writer);

    if written > 0 {
        unsafe { *off += written as usize };
    }

    written
}

#[no_mangle]
pub extern "C" fn tb_rs_open() -> *mut DebugfsState {
    pr_info!("tb_rs_open\n");
    match Box::try_new(DebugfsState {
        buf: None,
        data: MyCollection {
            num_objects: 7,
        }
    }) {
        Ok(s) => Box::into_raw(s),
        Err(_) => core::ptr::null::<DebugfsState>() as *mut DebugfsState
    }
}

#[no_mangle]
pub extern "C" fn tb_rs_release(private_data: *mut DebugfsState) {
    pr_info!("tb_rs_release\n");
    let _ = unsafe { Box::from_raw(private_data) };
}
