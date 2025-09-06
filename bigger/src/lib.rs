use std::collections::HashMap;


pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut bigger:i32 = 0;
    for (_key, value) in h {
        if value > bigger {
            bigger = value
        }
    }
    return bigger
}