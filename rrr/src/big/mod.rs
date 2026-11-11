use bitarray::BitArray;

mod prefix_sum;
use prefix_sum::PrefixSum;

mod ptables;
use ptables::PTables;

use crate::BitRankSelect;

pub struct BigRRR {
    fi: PrefixSum, // w(i) = f(i+1) − f(i)
    qi: PrefixSum, // s(i) = q(i+1) − q(i)
    pi: BitArray,  // p(i) and table(w(i), p(i))

    tables: ptables::PTables,

    bits: usize,
    bits_for_block: u8,
}

impl BigRRR {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() > 0 && bytes.len() <= (usize::MAX << 3));

        let bits = bytes.len() * 8;

        let bits_for_block = if bits.count_ones() == 1 {
            bits.ilog2()
        } else {
            bits.ilog2() + 1
        } as usize;
        let blocks = bits.div_ceil(bits_for_block);

        let w_len = if bits_for_block.count_ones() == 1 {
            bits_for_block.ilog2()
        } else {
            bits_for_block.ilog2() + 1
        } as usize;

        let mut fi = PrefixSum::builder(bits, blocks);
        let mut qi = PrefixSum::builder(1 << (bits_for_block / 2) + 1, blocks);
        let mut pi = BitArray::builder();

        let mut tables = PTables::builder(w_len, blocks);

        let mut index = 0;
        loop {
            if index >= bits {
                break;
            }

            let block_x = bitarray::raw::read_bits(bytes, index, bits_for_block);

            let wx = block_x.count_ones();
            let (sx, px) = tables.register_bits(wx as usize, block_x);

            fi.push(wx as usize);
            qi.push(sx as usize);
            pi.push(sx, px);

            index += bits_for_block;
        }

        Self {
            fi: fi.build(),
            qi: qi.build(),
            pi: pi.build(),

            tables: tables.build(),

            bits,
            bits_for_block: bits_for_block as u8,
        }
    }
}

impl BitRankSelect for BigRRR {
    fn total_bits(&self) -> usize {
        self.bits
    }

    fn rank_1(&self, i: usize) -> usize {
        let x = i / self.bits_for_block as usize;
        let xu = x * self.bits_for_block as usize;

        let fx = self.fi.get(x);
        let wx = self.fi.get(x + 1) - fx;

        let qx = self.qi.get(x);
        let sx = self.qi.get(x + 1) - qx;

        let px_pointer = self.pi.read_many(qx, sx);
        let px_value = self.tables.get_bits(wx, px_pointer);

        let pi_rank = (px_value >> (self.bits_for_block as usize - (i - xu) + 1)).count_ones();
        fx + pi_rank as usize
    }

    fn rank_0(&self, i: usize) -> usize {
        i.min(self.total_bits()) - self.rank_1(i)
    }

    fn access(&self, i: usize) -> bool {
        assert!(i < self.total_bits());

        self.rank_1(i + 1) - self.rank_1(i) == 1
    }

    fn select_1(&self, i: usize) -> Option<usize> {
        assert!(i != 0);
        let n = self.total_bits();

        if self.rank_1(n - 1) < i {
            return None;
        };

        let mut left = 0;
        let mut right = n - 1;
        while right > 1 + left {
            let mid = (right + left) / 2;
            if self.rank_1(mid) >= i {
                right = mid;
            } else {
                left = mid;
            }
        }
        Some(right)
    }

    fn select_0(&self, i: usize) -> Option<usize> {
        assert!(i != 0);
        let n = self.total_bits();

        if self.rank_0(n - 1) < i {
            return None;
        };

        let mut left = 0;
        let mut right = n - 1;
        while right > 1 + left {
            let mid = (right + left) / 2;
            if self.rank_0(mid) >= i {
                right = mid;
            } else {
                left = mid;
            }
        }
        Some(right)
    }
}
