mod big;
pub use big::BigRRR;

mod tiny;
pub use tiny::TinyRRR;

pub trait BitRankSelect {
    fn total_bits(&self) -> usize;

    /// rank(S, i) = fx + T[wx][px][i − xu],
    fn rank_1(&self, i: usize) -> usize;

    fn rank_0(&self, i: usize) -> usize {
        i - self.rank_1(i)
    }

    fn access(&self, i: usize) -> bool;

    fn select_1(&self, i: usize) -> Option<usize>;

    fn select_0(&self, i: usize) -> Option<usize>;
}

macro_rules! impl_bit_rank_select {
    ($ty:ty, $bits:expr, $half:ty, $half_bits:expr) => {
        impl BitRankSelect for $ty {
            #[inline]
            fn total_bits(&self) -> usize {
                $bits
            }

            #[inline]
            fn rank_1(&self, i: usize) -> usize {
                match i {
                    0 => 0,
                    $bits.. => self.count_ones() as usize,
                    _ => (self & ((1 << i) - 1)).count_ones() as usize,
                }
            }

            #[inline]
            fn rank_0(&self, i: usize) -> usize {
                i.min($bits) - self.rank_1(i)
            }

            #[inline]
            fn access(&self, i: usize) -> bool {
                assert!(i < $bits);

                (*self >> i) & 1 == 1
            }

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

impl BitRankSelect for usize {
    #[inline]
    fn total_bits(&self) -> usize {
        usize::BITS as usize
    }

    #[inline]
    fn rank_1(&self, i: usize) -> usize {
        const BITS: usize = usize::BITS as usize;

        match i {
            0 => 0,
            BITS.. => self.count_ones() as usize,
            _ => (self & ((1 << i as usize) - 1)).count_ones() as usize,
        }
    }

    #[inline]
    fn rank_0(&self, i: usize) -> usize {
        i.min(self.total_bits()) - self.rank_1(i)
    }

    #[inline]
    fn access(&self, i: usize) -> bool {
        assert!(i < 64);

        (*self >> i) & 1 == 1
    }

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

impl BitRankSelect for u8 {
    #[inline]
    fn total_bits(&self) -> usize {
        8
    }

    #[inline]
    fn rank_1(&self, i: usize) -> usize {
        match i {
            0 => 0,
            8.. => self.count_ones() as usize,
            _ => (self & ((1 << i) - 1)).count_ones() as usize,
        }
    }

    #[inline]
    fn rank_0(&self, i: usize) -> usize {
        i.min(8) - self.rank_1(i)
    }

    #[inline]
    fn access(&self, i: usize) -> bool {
        assert!(i < 8);

        (*self >> i) & 1 == 1
    }

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
