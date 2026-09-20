pub trait Word {
    type Alphabet: Copy + Ord;

    fn symbols(&self) -> impl Iterator<Item = Self::Alphabet> + '_;

    fn size(&self) -> usize;
    fn size_of(&self, symbol: Self::Alphabet) -> usize;
    fn size_of_symbol(&self, symbol: Symbol<Self::Alphabet>) -> usize {
        match symbol {
            Symbol::Char(c) => self.size_of(c),
            Symbol::Terminal => 1,
        }
    }

    fn try_get(&self, index: usize) -> Option<Self::Alphabet>;
    fn get(&self, index: usize) -> Symbol<Self::Alphabet> {
        self.try_get(index)
            .map(Symbol::Char)
            .unwrap_or(Symbol::Terminal)
    }
}

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

impl Word for str {
    type Alphabet = char;

    fn symbols(&self) -> impl Iterator<Item = char> + '_ {
        self.chars()
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn size_of(&self, symbol: char) -> usize {
        symbol.len_utf8()
    }

    fn try_get(&self, index: usize) -> Option<char> {
        self[index..].chars().next()
    }
}

impl<T: Copy + Ord> Word for [T] {
    type Alphabet = T;

    fn symbols(&self) -> impl Iterator<Item = T> + '_ {
        self.iter().copied()
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn size_of(&self, _: T) -> usize {
        1
    }

    fn try_get(&self, index: usize) -> Option<T> {
        self.get(index).copied()
    }
}
