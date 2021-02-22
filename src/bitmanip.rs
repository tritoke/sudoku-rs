/// A trait representing some common bit manipulations
pub trait BitManip {
    fn set_bit(&mut self, n: u32);
    fn flip_bit(&mut self, n: u32);
    fn test_bit(&self, n: u32) -> bool;
    fn clear_bit(&mut self, n: u32);
}

macro_rules! bitmanip_impl {
    ($t:ty) => {
        impl BitManip for $t {
            #[inline]
            fn set_bit(&mut self, n: u32) {
                *self |= 1 << n;
            }

            #[inline]
            fn flip_bit(&mut self, n: u32) {
                *self ^= 1 << n;
            }

            #[inline]
            fn test_bit(&self, n: u32) -> bool {
                (*self & (1 << n)) == (1 << n)
            }

            #[inline]
            fn clear_bit(&mut self, n: u32) {
                *self &= !(1 << n);
            }
        }
    };
}

macro_rules! bitmanip_impl_all {
    ($($t:ty)*) => ($(
        bitmanip_impl! { $t }
    )*)
}

bitmanip_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 isize i128 }
