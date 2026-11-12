pub unsafe trait Word: Ord {
    type Alphabet: Copy + Ord;

    /// The suffix returned must be a valid suffix that init in start position, if start is a valid position of a letter in the word.
    /// Must garantee that if self.size() - start >= 0, than the returned suffix must have the size = (self.size() - start).
    fn suffix(&self, start: usize) -> &Self;

    /// The prefix returned must be a valid prefix that finish in end position, if end is a valid position of a letter in the word.
    /// Must garantee that if self.size() >= start, than the returned prefix must have the size = end.
    fn prefix(&self, end: usize) -> &Self;

    /// A iterator for every letter in the word
    fn symbols(&self) -> impl Iterator<Item = Self::Alphabet> + '_;

    /// The length of the word
    fn size(&self) -> usize;

    /// The size of a letter in the word
    /// Must guarantee that the implementation of self.try_symbol_at is always valid.
    fn size_of(&self, symbol: Self::Alphabet) -> usize;

    fn size_of_symbol(&self, symbol: Symbol<Self::Alphabet>) -> usize {
        match symbol {
            Symbol::Char(c) => self.size_of(c),
            Symbol::Terminal => 0,
        }
    }

    /// Get the letter in the index position
    /// The next code must never panic:
    /// `
    /// if let Some(cur) = self.try_symbol(i) && i + self.size_of(cur) < self.size() {
    ///     let next = self.try_symbol_at(i + self.size_of(cur)).unwrap();
    /// }
    /// `
    fn try_symbol_at(&self, index: usize) -> Option<Self::Alphabet>;
    fn symbol_at(&self, index: usize) -> Symbol<Self::Alphabet> {
        self.try_symbol_at(index)
            .map(Symbol::Char)
            .unwrap_or(Symbol::Terminal)
    }
}

/// - Safety
///     For every Word w -> Word::symbols(w).count() == Word::size(w)
pub unsafe trait UniformeWord: Word {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol<T> {
    Terminal,
    Char(T),
}

impl<T: std::fmt::Display> std::fmt::Display for Symbol<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Char(c) => write!(f, "{c}"),
            Symbol::Terminal => write!(f, "$"),
        }
    }
}

unsafe impl Word for str {
    type Alphabet = char;

    #[inline]
    fn suffix(&self, start: usize) -> &Self {
        &self[start..]
    }

    #[inline]
    fn prefix(&self, end: usize) -> &Self {
        &self[..end]
    }

    #[inline]
    fn symbols(&self) -> impl Iterator<Item = char> + '_ {
        self.chars()
    }

    #[inline]
    fn size(&self) -> usize {
        self.len()
    }

    #[inline]
    fn size_of(&self, symbol: char) -> usize {
        symbol.len_utf8()
    }

    #[inline]
    fn try_symbol_at(&self, index: usize) -> Option<char> {
        self[index..].chars().next()
    }
}

unsafe impl<T: Copy + Ord> Word for [T] {
    type Alphabet = T;
    #[inline]
    fn suffix(&self, start: usize) -> &Self {
        &self[start..]
    }

    #[inline]
    fn prefix(&self, end: usize) -> &Self {
        &self[..end]
    }

    #[inline]
    fn symbols(&self) -> impl Iterator<Item = T> + '_ {
        self.iter().copied()
    }

    #[inline]
    fn size(&self) -> usize {
        self.len()
    }

    #[inline]
    fn size_of(&self, _: T) -> usize {
        1
    }

    #[inline]
    fn try_symbol_at(&self, index: usize) -> Option<T> {
        self.get(index).copied()
    }
}

unsafe impl<T: Copy + Ord> UniformeWord for [T] {}
