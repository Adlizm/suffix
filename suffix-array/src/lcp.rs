use suffix_tree::word::Word;

pub fn compute_lcp<T>(word: &T, sa: &[usize]) -> Vec<usize>
where
    T: Word + ?Sized,
{
    let n = word.size();

    if n == 0 {
        return Vec::new();
    }

    // rank[i] = posicao do sufixo i no Suffix Array
    let mut rank = vec![0usize; n];
    for (i, &s) in sa.iter().enumerate() {
        rank[s] = i;
    }

    let mut lcp = vec![0usize; sa.len()];
    let mut h: usize = 0;

    let mut i = 0;
    let mut hs = vec![];
    for c in word.symbols() {
        if rank[i] > 0 {
            let j = sa[rank[i] - 1]; // sufixo anterior no SA

            // Compara caracteres a partir da posicao h
            while i + h < n && j + h < n {
                if let Some(ih) = word.try_symbol_at(i + h)
                    && let Some(jh) = word.try_symbol_at(j + h)
                    && ih == jh
                {
                    h += word.size_of(ih);
                    hs.push(ih);
                } else {
                    break;
                }
            }

            lcp[rank[i]] = h;

            if h > 0 {
                h -= word.size_of(hs.pop().unwrap()); // propriedade de Kasai: h diminui no maximo 1
            }
        } else {
            h = 0;
            hs.clear();
        }

        i += word.size_of(c);
    }

    lcp
}
