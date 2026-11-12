pub struct BitArrayBuilder {
    bytes: Vec<u8>,
    len: usize,
}

impl BitArrayBuilder {
    pub(crate) fn new() -> Self {
        Self {
            bytes: Vec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, len: usize, bits: usize) {
        if self.len + len > self.bytes.len() * 8 {
            self.bytes.resize((self.len + len).div_ceil(8), 0);
        }

        super::raw::write_bits(&mut self.bytes, self.len, len, bits);
    }

    pub fn build(mut self) -> super::BitArray {
        self.bytes.truncate(self.len.div_ceil(8));

        let len = self.len;
        let bytes = self.bytes.into_boxed_slice();

        super::BitArray { bytes, len }
    }
}
