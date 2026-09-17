pub struct SuffixArray<'a> {
    word: &'a str,
    data: Vec<usize>,
}

impl<'a> SuffixArray<'a> {
    pub fn new(_word: &'a str) -> Self {
        todo!()
    }

    pub fn src(&self) -> &str {
        self.word
    }

    pub fn data(&self) -> &[usize] {
        &self.data
    }

    pub fn suffix(&self, i: usize) -> &'a str {
        &self.word[self.data[i]..]
    }
}
