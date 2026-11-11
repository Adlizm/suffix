use bitarray::raw::{read_bits, write_bits};

pub struct PrefixSumBuilder {
    raw: Vec<u8>,
    len: usize,
    remain: usize,

    sum: usize,
    saved_sum: usize,

    size_block: u16, // size in bits by a block
    size_max: u8,    // size in bits occupied by the first number in each block
    size_i: u8,      // size in bits occupied any non-first number in each block

    numbers_per_block: u8,
}

impl PrefixSumBuilder {
    pub fn push(&mut self, value: usize) {
        assert!(self.remain > 0, "this has already full");

        let index = self.len;
        let block = index / self.numbers_per_block as usize;
        let block_index = index % self.numbers_per_block as usize;

        let block_index_start = self.size_block as usize * block;

        self.sum += value;

        if block_index == 0 {
            self.saved_sum = self.sum;
            write_bits(
                &mut self.raw,
                block_index_start,
                self.size_max as usize,
                self.sum,
            );
        } else {
            let value = self.sum - self.saved_sum;
            let max = (1 << (self.size_i as u32 + 1)) - 1;

            assert!(value <= max, "block sum can't be greater than {max}");

            let index = block_index_start + block_index * self.size_i as usize;
            write_bits(&mut self.raw, index, self.size_i as usize, value);
        }

        self.remain -= 1;
        self.len += 1;
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
            raw: vec![0; blocks * size_block],
            len: 0,
            remain: max_numbers,

            sum: 0,
            saved_sum: 0,

            size_block: size_block as u16,
            size_max: size_max as u8,
            size_i: size_i as u8,

            numbers_per_block: numbers_per_block as u8,
        }
    }

    pub fn get(&self, index: usize) -> usize {
        assert!(index <= self.len);
        if index == 0 {
            return 0;
        }

        let numbers_per_block = 1 + (self.size_block - self.size_max as u16) / self.size_i as u16;

        let index = index - 1;
        let block = index / numbers_per_block as usize;
        let block_index = index % numbers_per_block as usize;

        let block_index_start = self.size_block as usize * block;

        let preffix_sum = read_bits(&self.raw, block_index_start, self.size_max as usize);
        let index_sum = if block_index == 0 {
            0
        } else {
            let index = block_index_start
                + (self.size_max as usize)
                + (self.size_i as usize) * (block_index - 1);

            read_bits(&self.raw, index, self.size_i as usize)
        };

        preffix_sum + index_sum
    }
}
