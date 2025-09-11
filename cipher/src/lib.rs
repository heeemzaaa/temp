#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut collect_ciphered = String::new();

    for c in original.chars() {
        if c.is_uppercase() {
            let ciphered_char = ('A' as u8 + 'Z' as u8) - c as u8;
            collect_ciphered.push(ciphered_char as char);
        } else if c.is_lowercase() {
            let ciphered_char = ('a' as u8 + 'z' as u8) - c as u8;
            collect_ciphered.push(ciphered_char as char);
        } else {
            collect_ciphered.push(c);
        }
    }
    if collect_ciphered == ciphered {
        return Ok(());
    }
    Err(CipherError {
        expected: collect_ciphered,
    })
}
