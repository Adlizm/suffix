pub trait BitRank {
    fn rank_1(&self, i: usize) -> usize;

    fn rank_0(&self, i: usize) -> usize {
        i - self.rank_1(i)
    }
}

macro_rules! impl_bit_rank {
    ($ty:ty, $bits:expr) => {
        impl BitRank for $ty {
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
        }
    };
}

impl_bit_rank!(u64, 64);
impl_bit_rank!(u32, 32);
impl_bit_rank!(u16, 16);
impl_bit_rank!(u8, 8);

impl BitRank for usize {
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
        i.min(usize::BITS as usize) - self.rank_1(i)
    }
}
