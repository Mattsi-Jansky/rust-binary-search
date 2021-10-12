pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let number_of_elements = array.len();
    let middle = number_of_elements / 2;
    if array[middle].eq(&key) {
        return Some(middle)
    }
    println!("Middle {}", middle);
    Some(0)
}
