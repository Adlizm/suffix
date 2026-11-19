pub trait BitSelect {
    fn select_1(&self, i: usize) -> Option<usize>;

    fn select_0(&self, i: usize) -> Option<usize>;
}

macro_rules! impl_bit_rank_select {
    ($ty:ty, $bits:expr, $half:ty, $half_bits:expr) => {
        impl BitSelect for $ty {
            #[inline]
            fn select_1(&self, i: usize) -> Option<usize> {
                if i == 0 {
                    return None;
                }

                let upper = (*self >> $half_bits) as $half;
                let lower = *self as $half;

                let upper_count = upper.count_ones() as usize;

                if i <= upper_count {
                    upper.select_1(i)
                } else {
                    lower.select_1(i - upper_count).map(|pos| pos + $half_bits)
                }
            }

            #[inline]
            fn select_0(&self, i: usize) -> Option<usize> {
                if i == 0 {
                    return None;
                }

                let upper = (*self >> $half_bits) as $half;
                let lower = *self as $half;

                let upper_count = upper.count_zeros() as usize;

                if i <= upper_count {
                    upper.select_0(i)
                } else {
                    lower.select_0(i - upper_count).map(|pos| pos + $half_bits)
                }
            }
        }
    };
}

impl_bit_rank_select!(u64, 64, u32, 32);
impl_bit_rank_select!(u32, 32, u16, 16);
impl_bit_rank_select!(u16, 16, u8, 8);

impl BitSelect for usize {
    #[inline]
    fn select_1(&self, i: usize) -> Option<usize> {
        #[cfg(target_pointer_width = "64")]
        return (*self as u64).select_1(i);

        #[cfg(target_pointer_width = "32")]
        return (*self as u32).select_1(i);

        #[cfg(target_pointer_width = "16")]
        return (*self as u16).select_1(i);
    }

    #[inline]
    fn select_0(&self, i: usize) -> Option<usize> {
        #[cfg(target_pointer_width = "64")]
        return (*self as u64).select_0(i);

        #[cfg(target_pointer_width = "32")]
        return (*self as u32).select_0(i);

        #[cfg(target_pointer_width = "16")]
        return (*self as u16).select_0(i);
    }
}

impl BitSelect for u8 {
    fn select_1(&self, i: usize) -> Option<usize> {
        if i == 0 || i > self.count_ones() as usize {
            return None;
        }

        let mut value = *self;

        for _ in 1..i {
            value &= value - 1;
        }

        Some(value.leading_zeros() as usize)
    }

    fn select_0(&self, i: usize) -> Option<usize> {
        if i == 0 || i > self.count_ones() as usize {
            return None;
        }

        let mut value = *self;

        for _ in 1..i {
            value &= value - 1;
        }

        Some(value.leading_zeros() as usize)
    }
}
