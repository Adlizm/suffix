use std::{collections::HashMap, hash::Hash};

use rrr::{BitAccess, BitRank, BitSelect, TinyRRR};

use word::UniformeWord;

enum Node<T> {
    Inner { bitarray: TinyRRR },
    Leaf { letter: T },
}

pub struct LetterData {
    count: usize,
    deep: usize,
    mask: usize,
}

pub struct WaveletTree<T: UniformeWord> {
    nodes: Vec<Node<T::Alphabet>>,
    letters: HashMap<T::Alphabet, LetterData>,
    len: usize,
}

impl<T: UniformeWord> WaveletTree<T>
where
    T::Alphabet: Hash,
{
    pub fn from(word: &T) -> Self {
        let mut letters: HashMap<T::Alphabet, LetterData> = HashMap::new();
        let mut uniques = 0;
        word.symbols().for_each(|c| {
            letters
                .entry(c)
                .and_modify(|e| e.count += 1)
                .or_insert_with(|| {
                    uniques += 1;
                    LetterData {
                        count: 1,
                        deep: 0,
                        mask: uniques - 1,
                    }
                });
        });

        if uniques == 0 {
            return Self {
                nodes: Default::default(),
                letters: Default::default(),
                len: 0,
            };
        }

        let mut nodes = Vec::with_capacity(2 * uniques - 1);
        let mut ranges = Vec::with_capacity(2 * uniques - 1);
        ranges.push(0..uniques);

        for mask in 1..=(2 * uniques - 1) {
            let range = ranges[mask - 1].clone();
            match range.len() {
                0 => unreachable!(),
                1 => {
                    let (letter, data) = letters
                        .iter_mut()
                        .find(|(_, data)| range.contains(&data.mask))
                        .unwrap();
                    data.deep = mask.ilog2() as usize;
                    data.mask = mask;

                    nodes.push(Node::Leaf { letter: *letter });
                }
                _ => {
                    let mid = (range.start + range.end) / 2;
                    let left = range.start..mid;
                    let right = mid..range.end;

                    let bitarray = TinyRRR::from_iter(word.symbols().filter_map(|l| {
                        let mask = letters.get(&l).unwrap().mask;
                        if left.contains(&mask) {
                            Some(false)
                        } else if right.contains(&mask) {
                            Some(true)
                        } else {
                            None
                        }
                    }));

                    nodes.push(Node::Inner { bitarray });
                    ranges.push(left);
                    ranges.push(right);
                }
            }
        }

        Self {
            nodes,
            letters,
            len: word.size(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn height(&self) -> usize {
        let unique = self.letters.len() as f64;
        let height = unique.log2().ceil() as usize + 1;

        height
    }

    pub fn access(&self, mut index: usize) -> T::Alphabet {
        assert!(index < self.len);

        let mut mask = 1usize;
        loop {
            let node = self
                .nodes
                .get(mask - 1)
                .expect("mask must point to a valid node");

            match node {
                Node::Inner { bitarray } => {
                    let bit = bitarray.access(index);

                    index = if bit {
                        bitarray.rank_1(index)
                    } else {
                        bitarray.rank_0(index)
                    };
                    mask = mask * 2 + bit as usize;
                }
                Node::Leaf { letter } => break *letter,
            }
        }
    }

    pub fn select(&self, letter: T::Alphabet, mut ith: usize) -> Option<usize> {
        let (mut mask, deep) = if let Some(data) = self.letters.get(&letter) {
            if ith > data.count {
                return None;
            }
            (data.mask, data.deep)
        } else {
            return None;
        };

        for _ in 1..=deep {
            mask = mask >> 1;

            let node = self.nodes.get(mask - 1).expect("mask must point to a valid node, otherwise something wrong with the implementation");
            match node {
                Node::Inner { bitarray } => {
                    let bit = mask & 1usize;

                    ith = if bit == 1 {
                        bitarray.select_1(ith)?
                    } else {
                        bitarray.select_0(ith)?
                    };
                }
                Node::Leaf { .. } => {
                    unreachable!(
                        "we already handle the case in the first 'if'.
                        Sinse we walking from deep to root, no more leaf must be found"
                    )
                }
            }
        }

        Some(ith)
    }

    pub fn rank(&self, letter: T::Alphabet, mut index: usize) -> usize {
        let (mask, deep, count) = if let Some(data) = self.letters.get(&letter) {
            (data.mask, data.deep, data.count)
        } else {
            return 0;
        };

        for deep in (0..=deep).rev() {
            let node_index = (mask >> deep) - 1;
            let node = self.nodes.get(node_index).expect("mask must point to a valid node, otherwise something wrong with the implementation");
            match node {
                Node::Inner { bitarray } => {
                    let bit = (mask >> deep - 1) & 1usize;

                    index = if bit == 1 {
                        bitarray.rank_1(index)
                    } else {
                        bitarray.rank_0(index)
                    };
                }
                Node::Leaf { letter: l } => {
                    debug_assert!(
                        *l == letter,
                        "find a different letter with the mask: {mask}"
                    );
                    return count.min(index);
                }
            }
        }

        unreachable!("alrealdy deep in the wavelet tree, but not found a leaf")
    }
}
