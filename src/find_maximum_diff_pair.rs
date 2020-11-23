fn find_maximum_diff_pair() -> i32 {
    let a = [1, 3, 8, 9, 10];
    let mut MAX = -10000;
    let len_a = a.len();

    for let mut i = 0 in 0..len_a {
        for let mut j = i+1..len_a {
            let diff_pairs = (a[i] - a[j]).abs()
            if diff_pairs > MAX {
                MAX = diff_pairs
                continue
            }
        }
    }
    return MAX;
}