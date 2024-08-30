fn main() {
    
    find_nemo("I am finding Nemo !");
    find_nemo("Nemo is me");
    find_nemo("who tf is nemo");
    find_nemo(" ");
    find_nemo("Try this neMo!");
    find_nemo("Try this newer rePetative neMo nemo nEmo");
    find_nemo("pump fake nemocracy");
    find_nemo("pump fake again ne:mo()crac!y but real NEMO is here.");
    find_nemo("hidden (n'e,m:o;)");
}

fn find_nemo(ocean:&str){

    let ocean = ocean.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'','!'][..], "");
    let str_vec: Vec<&str> = ocean.split(' ').collect();
    for (x, i) in str_vec.iter().enumerate() {
        if i.to_lowercase() == "nemo"{
            println!("I found Nemo at {}", x + 1);
            return;
        }else if i.to_lowercase().contains("nemo") && i.to_lowercase() != "nemo" {
            println!("⚠️ Fake Nemo alert at {} ⚠️", x + 1);
        }
    };
    println!("I can't find Nemo");
    return;
 }