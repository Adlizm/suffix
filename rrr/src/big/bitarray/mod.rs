mod builder;

pub use builder::BitArrayBuilder;

pub struct BitArray {
    bytes: Box<[u8]>,
    len: usize,
}

impl BitArray {
    pub fn builder() -> builder::BitArrayBuilder {
        builder::BitArrayBuilder::new()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec().into_boxed_slice(),
            len: bytes.len() * 8,
        }
    }

    pub fn with_length(len: usize, value: bool) -> Self {
        let mut data = Vec::with_capacity(len.div_ceil(8));

        let value = if value { u8::MAX } else { 0 };
        data.fill(value);

        Self {
            bytes: data.into_boxed_slice(),
            len,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> bool {
        raw::read_bits(&self.bytes, index, 1) == 1
    }

    pub fn set(&mut self, index: usize, bit: bool) {
        raw::write_bits(&mut self.bytes, index, 1, bit as usize)
    }

    pub fn read_many(&self, start: usize, len: usize) -> usize {
        raw::read_bits(&self.bytes, start, len)
    }

    pub fn write_many(&mut self, start: usize, len: usize, bits: usize) {
        raw::write_bits(&mut self.bytes, start, len, bits);
    }
}

pub mod raw {
    pub fn read_bits(src: &[u8], start: usize, len: usize) -> usize {
        assert!(len <= 64);
        assert!(start + len <= src.len() * 8);

        let mut pos = start;
        let mut remaining = len;
        let mut value = 0usize;

        while remaining > 0 {
            let offset = pos % 8;
            let available = 8 - offset;
            let take = remaining.min(available);

            let shift = available - take;
            let mask = ((1u16 << take) - 1) as u8;

            let chunk = (src[pos / 8] >> shift) & mask;

            value = (value << take) | chunk as usize;

            pos += take;
            remaining -= take;
        }

        value
    }

    pub fn write_bits(dst: &mut [u8], start: usize, len: usize, bits: usize) {
        assert!(len <= 64);
        assert!(start + len <= dst.len() * 8);

        let mut pos = start;
        let mut remaining = len;

        while remaining > 0 {
            let offset = pos % 8;
            let available = 8 - offset;
            let take = remaining.min(available);

            let shift = remaining - take;
            let mask = ((1u16 << take) - 1) as u8;
            let chunk = ((bits >> shift) as u8) & mask;

            let dst_shift = available - take;
            let dst_mask = mask << dst_shift;

            let byte = &mut dst[pos / 8];
            *byte = (*byte & !dst_mask) | (chunk << dst_shift);

            pos += take;
            remaining -= take;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_bits_at_byte_boundary() {
        let mut buf = [0u8; 2];

        raw::write_bits(&mut buf, 0, 4, 0b1011);

        assert_eq!(buf, [0b1011_0000, 0b0000_0000]);
    }

    #[test]
    fn writes_bits_with_offset() {
        let mut buf = [0u8; 2];

        raw::write_bits(&mut buf, 2, 3, 0b101);

        assert_eq!(buf, [0b0010_1000, 0b0000_0000]);
    }

    #[test]
    fn writes_bits_across_byte_boundary() {
        let mut buf = [0u8; 2];

        raw::write_bits(&mut buf, 6, 4, 0b1011);

        assert_eq!(buf, [0b0000_0010, 0b1100_0000]);
    }

    #[test]
    fn preserves_existing_bits() {
        let mut buf = [0b1111_1111, 0b1111_1111];

        raw::write_bits(&mut buf, 2, 3, 0b000);

        assert_eq!(buf, [0b1100_0111, 0b1111_1111]);
    }

    #[test]
    fn overwrites_existing_bits() {
        let mut buf = [0b0000_0000, 0b0000_0000];

        raw::write_bits(&mut buf, 2, 3, 0b111);

        assert_eq!(buf, [0b0011_1000, 0]);
    }

    #[test]
    fn writes_all_bits_in_byte() {
        let mut buf = [0u8; 1];

        raw::write_bits(&mut buf, 0, 8, 0b1010_1010);

        assert_eq!(buf, [0b1010_1010]);
    }

    #[test]
    fn writes_bits_at_last_position() {
        let mut buf = [0u8; 1];

        raw::write_bits(&mut buf, 7, 1, 1);

        assert_eq!(buf, [0b0000_0001]);
    }

    #[test]
    fn zero_length_does_nothing() {
        let mut buf = [0b1010_1010];

        raw::write_bits(&mut buf, 0, 0, 0);

        assert_eq!(buf, [0b1010_1010]);
    }

    #[test]
    fn reads_bits_at_byte_boundary() {
        let buf = [0b1011_0000, 0b0000_0000];

        assert_eq!(raw::read_bits(&buf, 0, 4), 0b1011);
    }

    #[test]
    fn reads_bits_across_byte_boundary() {
        let buf = [0b00000010, 0b11000000];

        assert_eq!(raw::read_bits(&buf, 6, 4), 0b1011);
    }

    #[test]
    fn reads_bits_preserving_bit_order() {
        let buf = [0b1101_0110];

        assert_eq!(raw::read_bits(&buf, 1, 5), 0b10101);
    }

    #[test]
    fn reads_zero_bits() {
        let buf = [0xFF];

        assert_eq!(raw::read_bits(&buf, 0, 0), 0);
    }
}
