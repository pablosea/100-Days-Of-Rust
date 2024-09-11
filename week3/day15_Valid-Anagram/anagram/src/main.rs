use std::collections::HashMap;

fn main() {

    let str1 = "anagram";
    let str2 ="nagaram";
    let s= "rat";
    let t = "car";
    let s2 = "bing";
    let s3 = "ginb";
    
    is_anagram(str1, str2);
    is_anagram(s,t);
    is_anagram2(s2, s3);
}

fn is_anagram(word1:&str ,word2: &str) -> bool {
    if word1.len() == word2.len(){
        for c in word1.chars().collect::<Vec<_>>(){
            if word2.chars().collect::<Vec<_>>().contains(&c){
                continue;
             }else {
                println!("missed one");
                return false;
            }
        } 
        true
    }else {
        false
    }
}

//attempting not to use vec
fn is_anagram2(s1:&str ,s2: &str) -> bool {

    let mut char_counts = HashMap::new();

    s2.chars().for_each(|c| *char_counts.entry(c).or_insert(0) +=1);

    let result = s1.chars().all(|c| {
        char_counts.entry(c).and_modify(|e| *e -= 1).or_insert(-1);
        *char_counts.get(&c).unwrap_or(&-1) >= 0
    }) && char_counts.values().all(|&count| count == 0);

    result
}