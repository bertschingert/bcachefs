#[repr(C)]
#[repr(align(8))]
#[derive(Default, Copy, Clone)]
pub struct bkey {
    pub u64s: __u8,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub type_: __u8,
    pub pad: [__u8; 1usize],
    pub version: bversion,
    pub size: __u32,
    pub p: bpos,
}
impl bkey {
    #[inline]
    pub fn format(&self) -> __u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u8) }
    }
    #[inline]
    pub fn set_format(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn needs_whiteout(&self) -> __u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_needs_whiteout(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        format: __u8,
        needs_whiteout: __u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let format: u8 = unsafe { ::core::mem::transmute(format) };
            format as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let needs_whiteout: u8 = unsafe { ::core::mem::transmute(needs_whiteout) };
            needs_whiteout as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[repr(align(8))]
#[derive(Default, Copy, Clone)]
pub struct bch_extent_ptr {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
impl bch_extent_ptr {
    #[inline]
    pub fn type_(&self) -> __u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_type(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn cached(&self) -> __u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_cached(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn unused(&self) -> __u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_unused(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn unwritten(&self) -> __u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_unwritten(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn offset(&self) -> __u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 44u8) as u64) }
    }
    #[inline]
    pub fn set_offset(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 44u8, val as u64)
        }
    }
    #[inline]
    pub fn dev(&self) -> __u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(48usize, 8u8) as u64) }
    }
    #[inline]
    pub fn set_dev(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(48usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn gen(&self) -> __u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(56usize, 8u8) as u64) }
    }
    #[inline]
    pub fn set_gen(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(56usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        type_: __u64,
        cached: __u64,
        unused: __u64,
        unwritten: __u64,
        offset: __u64,
        dev: __u64,
        gen: __u64,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let type_: u64 = unsafe { ::core::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let cached: u64 = unsafe { ::core::mem::transmute(cached) };
            cached as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let unused: u64 = unsafe { ::core::mem::transmute(unused) };
            unused as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let unwritten: u64 = unsafe { ::core::mem::transmute(unwritten) };
            unwritten as u64
        });
        __bindgen_bitfield_unit.set(4usize, 44u8, {
            let offset: u64 = unsafe { ::core::mem::transmute(offset) };
            offset as u64
        });
        __bindgen_bitfield_unit.set(48usize, 8u8, {
            let dev: u64 = unsafe { ::core::mem::transmute(dev) };
            dev as u64
        });
        __bindgen_bitfield_unit.set(56usize, 8u8, {
            let gen: u64 = unsafe { ::core::mem::transmute(gen) };
            gen as u64
        });
        __bindgen_bitfield_unit
    }
}
