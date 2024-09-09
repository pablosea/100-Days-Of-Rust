fn main() {
    let items1: Vec<i32> = [2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2 ].to_vec();
    let bags1: i32 = 4;

    let items2: Vec<i32> = vec![2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2];
    

    can_fit(items1, bags1);
    can_fit(items2, bags1);
}

fn can_fit(items: Vec<i32>, bags: i32) -> bool{

    let capcity: i32 = bags * 10;
    let item_weight: i32 = items.iter().sum();

    if item_weight > capcity{
        println!("nuh");
        return false;
    }else {
        println!("yuh");
        return true;
    }
    
}
