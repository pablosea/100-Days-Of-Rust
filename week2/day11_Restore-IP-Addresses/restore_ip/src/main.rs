struct IpRestorer;

impl IpRestorer {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        
        Self::backtrack(&chars, 0, 0, &mut String::new(), &mut result);
        
        result
    }

    fn backtrack(
                chars: &[char], 
                index: usize, 
                segment: usize, 
                current: &mut String, 
                result: &mut Vec<String>) 
            {

        if index == chars.len() && segment == 4{
            result.push(current.clone());
            return;
        }
        
        if segment > 3 || (index == chars.len() && segment != 3) {
            return;
        }

        let original_len = current.len();
        if segment > 0 {
            current.push('.');
        }

        for i in 1..=3 {
            if index + i > chars.len() {
                break;
            }
            
            let octet: String = chars[index..index + i].iter().collect();
            
            if Self::is_valid_octet(&octet) {
                current.push_str(&octet);
                Self::backtrack(chars, index + i, segment + 1, current, result);
                current.truncate(original_len + if segment > 0 { 1 } else { 0 });
            }
        }
    }

    fn is_valid_octet(s: &str) -> bool {
        if s.len() > 1 && s.starts_with("0") {
            return false;
        }
        
        match s.parse::<u8>() {
            Ok(num) => num <= 255,
            Err(_) => false,
        }
    }
}

fn main() {
    let s = String::from("25525511135");
    let s2: String = String::from("101023");
    let result = IpRestorer::restore_ip_addresses(s);
    let result2 = IpRestorer::restore_ip_addresses(s2);
    println!("{:?} \n {:?}", result, result2);
}