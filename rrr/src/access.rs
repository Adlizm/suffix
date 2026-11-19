pub trait BitAccess {
    fn access(&self, i: usize) -> bool;
}

macro_rules! impl_bit_access {
    ($ty:ty, $bits:expr) => {
        impl BitAccess for $ty {
            #[inline]
            fn access(&self, i: usize) -> bool {
                assert!(i < $bits);

                (*self >> i) & 1 == 1
            }
        }
    };
}

impl_bit_access!(u64, 64);
impl_bit_access!(u32, 32);
impl_bit_access!(u16, 16);
impl_bit_access!(u8, 8);

impl BitAccess for usize {
    #[inline]
    fn access(&self, i: usize) -> bool {
        #[cfg(target_pointer_width = "64")]
        return (*self as u64).access(i);

        #[cfg(target_pointer_width = "32")]
        return (*self as u32).access(i);

        #[cfg(target_pointer_width = "16")]
        return (*self as u16).access(i);
    }
}
