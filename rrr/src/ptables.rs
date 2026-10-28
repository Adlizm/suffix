pub struct PTables {}

impl PTables {
    pub fn get_raw_pointer(&self, index: usize, size: usize) -> usize {
        todo!()
    }
    pub fn get_bits(&self, ones: usize, pointer: usize) -> usize {
        todo!()
    }

    pub fn builder(max_bits_ones: usize, max_values: usize) -> PTablesBuilder {
        PTablesBuilder {}
    }
}

pub struct PTablesBuilder {}

impl PTablesBuilder {
    pub fn register_bits(&mut self, ones: usize, bits: usize) -> usize {
        todo!()
    }

    pub fn build(self) -> PTables {
        PTables {}
    }
}
