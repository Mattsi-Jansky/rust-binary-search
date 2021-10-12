pub fn find(haystack: &[i32], needle: i32) -> Option<usize> {
    let number_of_elements = haystack.len();
    let middle = number_of_elements / 2;
    if haystack[middle].eq(&needle) {
        return Some(middle)
    } else if haystack[haystack.len()-1] == needle {
        return Some(haystack.len()-1)
    }
    println!("Middle {}", middle);
    Some(0)
}
