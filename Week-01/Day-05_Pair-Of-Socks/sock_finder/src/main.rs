fn main() {
    find_socks("AA");
    find_socks("ABABC");
    find_socks("CABBACCCC");
    find_socks("");
    find_socks("AAAAAAAAAA");
    find_socks("WIFHDFHBFYGFIOIHKNEHFDABFVJHVSJSGYBSHCDCPDPSDLDKSJSJDHFGSVCSYJFJAFBDYBFSDKGUBRGSDVFASDAVTSHDVHUIFHBSBYETFEDEYGFISUHGIUSNGDSHBVDVCYAKCNAIUEFBAUNFIA");
    find_socks("CVBHGFTTYUPOLOLK95887975537OOLKM");
}

fn find_socks(sock_drawer: &str){
    let mut matched: i32 = 0;
    let mut tot_matched: i32 = 0;
    let mut pair: i32 = 0;
    let mut socks: Vec<_> = sock_drawer.chars().collect();
    socks.sort();

    for (x, i) in socks.iter().enumerate(){
        //println!("{i}");
        if x > 0{
            if i == &socks[x-1]{
                matched += 1;
                tot_matched +=1;
                //println!("match! {}: total matched {}",matched, tot_matched);
                if matched == 1 && x==1{
                        pair += 1;
                        //println!("added to pair");
                }else if matched % 2 == 0{
                        pair += 1;
                        //println!("added to pair in mod");
                }
            }else{
              //println!("new letter");
              matched = 1;
            }
        }
    }
    println!("total pairs: {}, total matches: {} ", pair, tot_matched);
}
