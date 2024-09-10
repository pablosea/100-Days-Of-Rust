use std::collections::HashMap;

fn main() {
    encrypt("bing".to_string());
    encrypt(String::from("banana"));
    encrypt("karaca".to_string());
    encrypt("burak".to_string());
    encrypt(String::from("alpaca"));
}

fn encrypt(word: String) {

    let vowel_map: HashMap<char, char> = HashMap::from([
        ('a', '0'),
        ('e', '1'),
        ('i', '2'),
        ('o', '2'),
        ('u', '3'),
    ]);

    let mut newwrd: String = word.chars()
                   .rev()
                   .map(|c| *vowel_map.get(&c).unwrap_or(&c))
                   .collect();

    newwrd.push_str("aca");
                   
    println!("{}", newwrd);
}