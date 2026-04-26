use std::collections::HashMap;

pub fn tokenizer(sentence: &String) -> HashMap<String, i32> {
    let full_sentence = &sentence;
    let list_words: Vec<&str> = full_sentence.split_whitespace().collect();

    let mut data_table = HashMap::new();

    for &x in &list_words {
        let mut count = 0; 

        for &y in &list_words {
            if x == y {
                count = count + 1
            }
        }

        data_table.insert(String::from(x), count);
    }

    return data_table;
}