pub struct PrefixSumBuilder {
    raw: Vec<u8>,
    len: usize,
    remain: usize,

    size_block: u16, // size in bits by a block
    size_max: u8,    // size in bits occupied by the first number in each block
    size_i: u8,      // size in bits occupied any non-first number in each block
}

impl PrefixSumBuilder {
    pub fn push(&mut self, value: usize) {
        assert!(self.remain > 0);
    }

    pub fn build(self) -> PrefixSum {
        PrefixSum {
            raw: self.raw.into_boxed_slice(),
            len: self.len,
            size_block: self.size_block,
            size_max: self.size_max,
            size_i: self.size_i,
        }
    }
}

pub struct PrefixSum {
    raw: Box<[u8]>,
    len: usize,

    size_block: u16, // size in bits by a block
    size_max: u8,    // size in bits occupied by the first number in each block
    size_i: u8,      // size in bits occupied any non-first number in each block
}

impl PrefixSum {
    pub fn builder(max_value: usize, max_numbers: usize) -> PrefixSumBuilder {
        let size_max = max_value.ilog2() + (max_value.count_ones() != 1) as u32;
        let size_i = size_max.ilog2() + (size_max.count_ones() != 1) as u32;

        let numbers_per_block = max_numbers.ilog2() + (max_numbers.count_ones() != 1) as u32;
        let size_block = (size_max + size_i * (numbers_per_block - 1)) as usize;
        let blocks = max_numbers.div_ceil(numbers_per_block as usize);

        PrefixSumBuilder {
            raw: Vec::with_capacity(blocks * size_block),
            len: 0,
            remain: max_numbers,

            size_block: size_block as u16,
            size_max: size_max as u8,
            size_i: size_i as u8,
        }
    }

    pub fn get(&self, index: usize) -> usize {
        assert!(index <= self.len);
        if index == 0 {
            return 0;
        }

        todo!()
    }
}
