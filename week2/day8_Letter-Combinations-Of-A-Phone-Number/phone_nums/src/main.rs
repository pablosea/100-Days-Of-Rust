use std::collections::HashMap;

fn main() {
    find_t9("23");
    find_t9("57q83ny(*u3901g93");
    find_t9("");
}

fn find_t9(nums: &str){

    let valid_chars: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];

    let charvec = nums.chars();
    let mut numvec = Vec::new();
    let mut lettervec = Vec::new();

    for i in charvec{
        if valid_chars.contains(&i){
            let b: String = i.to_string();
            numvec.push(b.parse::<i32>().unwrap());
        } else {
            //println!("invalid character input {i} is not supported and has been removed. \n");
        }
    }

    let t9: HashMap<i32, &str> = HashMap::from([
        (1, ""),
        (2, "abc"),
        (3, "def"),
        (4, "ghi"),
        (5, "jkl"),
        (6, "mno"),
        (7, "pqrs"),
        (8, "tuv"),
        (9, "wxyz"),
        (0, "")
        ]
    );

    for nummy in numvec {
        match t9.get(&nummy){
            Some(letters) => lettervec.push(letters),
            None => println!("Invalid input bucko.")
        };
    }

    println!("{:?}", lettervec)

}

