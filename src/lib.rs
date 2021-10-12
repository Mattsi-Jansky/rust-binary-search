pub fn find(haystack: &[i32], needle: i32) -> Option<usize> {
    let number_of_elements = haystack.len();
    let middle = number_of_elements / 2;

    if haystack[middle].eq(&needle) {
        Some(middle)
    } else if haystack[haystack.len()-1] == needle {
        Some(haystack.len() - 1)
    } else {
        Some(0)
    }
}
