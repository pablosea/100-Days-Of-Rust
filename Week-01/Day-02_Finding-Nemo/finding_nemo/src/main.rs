fn main() {
    find_nemo("I am finding Nemo !");
    find_nemo("Nemo is me");
    find_nemo("who tf is nemo");
    find_nemo(" ");
    find_nemo("Try this neMo!");
    find_nemo("Try this newer rePetative neMo nemo nEmo");
}

fn find_nemo(ocean:&str){
    
    let ocean = ocean.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'','!'][..], "");
    let str_vec: Vec<&str> = ocean.split(' ').collect();

    for (x, i) in str_vec.iter().enumerate() {
        if i.to_lowercase() == "nemo"{
            println!("I found Nemo at {}", x + 1);
            return;
        }
    };
    println!("I can't find Nemo");
    return;
 }

 