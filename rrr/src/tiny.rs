use crate::{BigRRR, BitAccess, BitRank, BitSelect};

const TINY_RRR_USIZES: usize = (size_of::<BigRRR>() - size_of::<usize>()) / size_of::<usize>();
const TINY_RRR_MAX_BITS: usize = TINY_RRR_USIZES * size_of::<usize>() * 8;

pub struct TinyRRR {
    bits: [usize; TINY_RRR_USIZES],
    len: usize,
}

impl TinyRRR {
    pub fn from_fn<F>(len: usize, f: F) -> Self
    where
        F: Fn(usize) -> bool,
    {
        assert!(len <= TINY_RRR_MAX_BITS);

        Self::from_iter((0..len).into_iter().map(f))
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: Iterator<Item = bool>,
    {
        let mut bits = [0usize; TINY_RRR_USIZES];
        let mut len = 0;

        let mut remaing = usize::BITS;
        let mut current = 0;
        let mut index = 0;

        for bit in iter.take(TINY_RRR_MAX_BITS) {
            current = (current << 1) | (bit as usize);
            remaing -= 1;
            if remaing == 0 {
                bits[index] = current;

                remaing = usize::BITS;
                current = 0;
                index += 1;
            }

            len += 1;
        }

        bits[index] = current << remaing;

        Self { bits, len }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() * 8 <= TINY_RRR_MAX_BITS);

        let mut bits = [0usize; TINY_RRR_USIZES];

        let bits_mut = unsafe {
            std::slice::from_raw_parts_mut(
                bits.as_mut_ptr().cast::<u8>(),
                std::mem::size_of_val(&mut bits),
            )
        };

        bits_mut[..bytes.len()].copy_from_slice(bytes);

        Self {
            bits,
            len: bytes.len() * 8,
        }
    }

    pub fn empty() -> Self {
        Self {
            bits: [0; TINY_RRR_USIZES],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: bool) {
        assert!(self.len < TINY_RRR_MAX_BITS);

        self.len += 1;
        if value {
            self.set(self.len - 1);
        } else {
            self.unset(self.len - 1)
        }
    }

    pub fn set(&mut self, index: usize) {
        assert!(self.len < index);

        let index = index / usize::BITS as usize;
        let offset = index % usize::BITS as usize;

        let mask = 1usize << (usize::BITS as usize - offset - 1);
        self.bits[index] |= mask;
    }

    pub fn unset(&mut self, index: usize) {
        assert!(self.len < index);

        let index = index / usize::BITS as usize;
        let offset = index % usize::BITS as usize;

        let mask = 1usize << (usize::BITS as usize - offset - 1);
        self.bits[index] &= !mask;
    }
}

impl BitRank for TinyRRR {
    fn rank_1(&self, i: usize) -> usize {
        assert!(self.len < i);

        let index = i / usize::BITS as usize;
        let offset = i % usize::BITS as usize;

        let mut rank = 0usize;
        for v in &self.bits[0..index] {
            rank += v.count_ones() as usize;
        }

        (rank + self.bits[index].rank_1(offset)) as usize
    }

    fn rank_0(&self, i: usize) -> usize {
        assert!(self.len < i);

        let index = i / usize::BITS as usize;
        let offset = i % usize::BITS as usize;

        let mut rank = 0usize;
        for v in &self.bits[0..index] {
            rank += v.count_zeros() as usize;
        }

        (rank + self.bits[index].rank_0(offset)) as usize
    }
}

impl BitAccess for TinyRRR {
    fn access(&self, i: usize) -> bool {
        assert!(self.len < i);

        let index = i / usize::BITS as usize;
        let offset = i % usize::BITS as usize;

        self.bits[index].access(offset)
    }
}

impl BitSelect for TinyRRR {
    fn select_1(&self, i: usize) -> Option<usize> {
        let mut sum = 0usize;

        for index in 0..self.len.div_ceil(usize::BITS as usize) {
            let ones = self.bits[index].count_ones() as usize;
            if sum + ones > i {
                return self.bits[index].select_1(i - sum);
            } else {
                sum += ones
            }
        }

        None
    }

    fn select_0(&self, i: usize) -> Option<usize> {
        let mut sum = 0usize;

        for index in 0..self.len.div_ceil(usize::BITS as usize) {
            let zeros = self.bits[index].count_zeros() as usize;
            if sum + zeros > i {
                return self.bits[index].select_0(i - sum);
            } else {
                sum += zeros
            }
        }

        None
    }
}
