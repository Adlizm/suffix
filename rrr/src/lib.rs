mod prefix_sum;
mod ptables;
mod utils;

pub struct RRR {
    fi: prefix_sum::PrefixSum, // w(i) = f(i+1) − f(i)
    qi: prefix_sum::PrefixSum, // s(i) = q(i+1) − q(i)
    pi: ptables::PTables,      // p(i) and table(w(i), p(i))

    bits: usize,
    bits_for_block: u8,
}

impl RRR {
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

        let mut fi = prefix_sum::PrefixSum::builder(bits, blocks);
        let mut qi = prefix_sum::PrefixSum::builder(1 << (bits_for_block / 2) + 1, blocks);
        let mut pi = ptables::PTables::builder(w_len, blocks);

        let mut index = 0;
        loop {
            if index >= bits {
                break;
            }

            let block_x = utils::read_bits(bytes, index, bits_for_block as u8);

            let wx = block_x.count_ones();
            let sx = pi.register_bits(wx as usize, block_x);

            fi.push(wx as usize);
            qi.push(sx as usize);

            index += bits_for_block;
        }

        Self {
            fi: fi.build(),
            qi: qi.build(),
            pi: pi.build(),

            bits,
            bits_for_block: bits_for_block as u8,
        }
    }

    pub fn access(&self, i: usize) -> bool {
        if i == 0 {
            self.rank(i) == 1
        } else {
            (self.rank(i) - self.rank(i - 1)) == 1
        }
    }

    /// rank(S, i) = fx + T[wx][px][i − xu],
    pub fn rank(&self, i: usize) -> usize {
        let x = i / self.bits_for_block as usize;
        let xu = x * self.bits_for_block as usize;

        let fx = self.fi.get(x);
        let wx = self.fi.get(x + 1) - fx;

        let qx = self.qi.get(x);
        let sx = self.qi.get(x + 1) - qx;

        let px_pointer = self.pi.get_raw_pointer(qx, sx);
        let px_value = self.pi.get_bits(wx, px_pointer);

        let pi_rank = (px_value >> (self.bits_for_block as usize - (i - xu) + 1)).count_ones();
        fx + pi_rank as usize
    }
    pub fn rank_0(&self, i: usize) -> usize {
        (i + 1) - self.rank(i)
    }

    pub fn select(&self, i: usize) -> Option<usize> {
        let n = self.bits;

        if self.rank(n - 1) < i {
            return None;
        };

        let mut left = 0;
        let mut right = n - 1;
        while right > 1 + left {
            let mid = (right + left) / 2;
            if self.rank(mid) >= i {
                right = mid;
            } else {
                left = mid;
            }
        }
        Some(right)
    }
    pub fn select_0(&self, i: usize) -> Option<usize> {
        let n = self.bits;

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
