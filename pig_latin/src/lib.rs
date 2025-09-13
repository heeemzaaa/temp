pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut res: String = String::new();
    let chars: Vec<char> = text.chars().collect();

    for v in vowels.iter() {
        if text.starts_with(*v) {
            res = text.to_string() + "ay";
            return res;
        }
    }

    for (i, c) in chars.iter().enumerate() {
        if vowels.contains(&c) {
            let after = &text[i..];
            let before = &text[0..i];
            res = after.to_string() + &before.to_string() + "ay";
            return res;
        }

        if chars.len() >= 3 && !vowels.contains(&chars[0]) && chars[1] == 'q' && chars[2] == 'u' {
            let rest: String = chars[3..].iter().collect(); 
            let moved: String = chars[..3].iter().collect(); 
            return rest + &moved + "ay";
        }
    }
    res
}
