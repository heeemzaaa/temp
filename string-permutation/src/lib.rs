pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut counter:usize = 0;
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                counter+=1;
                break;
            }
        }
    }
    if counter == s2.len() {
        return true;
    }
    false
}
