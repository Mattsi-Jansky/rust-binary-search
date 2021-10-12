pub fn find(haystack: &[i32], needle: i32) -> Option<usize> {
    let number_of_elements = haystack.len();
    let mut index = 0;
    let mut length = number_of_elements;
    let mut count = 0;

    while count < 15 {
        println!("Loop count {} with index {} and length {}", count, index, length);
        let middle = index + (length / 2);
        println!("Middle: {}", middle);
        if haystack[middle].eq(&needle) {
            println!("Found correct value, returning {}", middle);
            return Some(middle)
        }
        if needle < haystack[middle] {
            println!("{} is less than {} therefore to to left side", needle, haystack[middle]);
            length = (length / 2);
        } else {
            println!("{} is greater than {} therefore to to right side", needle, haystack[middle]);
            index = middle + 1; //5
            length = (length / 2)
        }

        if length == 0 {
            return None
        }
        count += 1;
    }
    Some(0)
}
