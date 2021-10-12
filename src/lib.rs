pub fn find(haystack: &[i32], needle: i32) -> Option<usize> {
    let mut index = 0;
    let mut length = haystack.len();

    while length > 0 {
        let half = (length / 2);
        let middle = index + half;
        if haystack[middle].eq(&needle) {
            return Some(middle)
        } else if needle > haystack[middle] {
            index = middle + 1;
        }
        length = half;
    }

    None
}
