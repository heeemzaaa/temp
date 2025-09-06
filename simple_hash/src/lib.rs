use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut res:HashMap<&'a str, usize> = HashMap::new();

    for word in words {
        let temp = res.get(word);

        match temp {
            Some(value) => {
                res.insert(word, value + 1);
            },
            _ => {
                res.insert(word, 1);
            }
        }
    }
    return res;
}


pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    let mut counter:usize = 0;
    for (_, _) in frequency_count {
        counter+=1;
    }
    return counter;
}