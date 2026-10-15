/// Compute the BWT of the input data.
pub fn bwt_transform(data: &[u8]) -> (Vec<u8>, usize) {
    if data.is_empty() {
        return (vec![], 0);
    }

    let n = data.len();
    let sa = suffix_array::SuffixArray::from_slice(data);
    let saa = sa.array();

    // Find the index of the original string in the sorted rotation list
    let mut orig_idx = 0;
    for (i, &start) in saa.iter().enumerate() {
        if start == 0 {
            orig_idx = i;
            break;
        }
    }

    // The BWT is the last column of the sorted rotation matrix
    let bwt: Vec<u8> = saa.iter().map(|&i| data[(i + n - 1) % n]).collect();

    (bwt, orig_idx)
}

/// Inverse BWT: recover original data from transformed data and original index.
pub fn bwt_inverse(transformed: &[u8], orig_idx: usize) -> Vec<u8> {
    if transformed.is_empty() {
        return vec![];
    }

    let n = transformed.len();

    // Count occurrences of each byte
    let mut count = vec![0usize; 256];
    for &b in transformed {
        count[b as usize] += 1;
    }

    // Cumulative counts (first occurrence of each byte in first column)
    let mut first_occurrence = vec![0usize; 256];
    let mut sum = 0;
    for i in 0..256 {
        first_occurrence[i] = sum;
        sum += count[i];
    }

    // Build LF-mapping: for each position in last column, find position in first column
    let mut lf = vec![0usize; n];
    let mut occ_count = vec![0usize; 256];
    for i in 0..n {
        let b = transformed[i] as usize;
        lf[i] = first_occurrence[b] + occ_count[b];
        occ_count[b] += 1;
    }

    // Reconstruct by following LF-mapping from orig_idx
    let mut result = vec![0u8; n];
    let mut idx = orig_idx;
    for i in (0..n).rev() {
        result[i] = transformed[idx];
        idx = lf[idx];
    }

    result
}
