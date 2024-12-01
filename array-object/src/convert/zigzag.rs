pub trait Zigzag {
    fn zigzag(self) -> Self;
    fn straight(self) -> Self;
}

macro_rules! impl_zigzag {
    ($ty:tt, $ty2:tt, $n:literal) => {
        impl Zigzag for $ty {
            fn zigzag(mut self) -> Self {
                self = (self << 1) ^ (self >> $n);
                self
            }
            fn straight(mut self) -> Self {
                self = (((self as $ty2) >> 1) as $ty) ^ (-(self & 1));
                self
            }
        }
    };
}

impl_zigzag!(i8, u8, 7);
impl_zigzag!(i16, u16, 15);
impl_zigzag!(i32, u32, 31);
impl_zigzag!(i64, u64, 63);
impl_zigzag!(i128, u128, 127);

#[cfg(target_pointer_width = "64")]
impl_zigzag!(isize, usize, 63);

#[cfg(target_pointer_width = "32")]
impl_zigzag!(isize, usize, 31);