fn main() {

    let landscape1: [i32; 6] = [1, 3, 5, 4, 3, 2];
    let landscape2: [i32;8] =[10, 9, 8, 7, 2, 3, 4, 5];
    let landscape3: [i32; 6] = [1, 2, 3, 2, 4, 1];
    let landscape4: [i32;5] = [-1, -1, 0, -1, -1];
    let landscape5: [i32;5] = [5, 4, 3, 2, 1];
    let landscape7: [i32;7] = [0, -1, -1, 0, -1, -1, 0]; 

    landscape_type(landscape1.to_vec());
    landscape_type(landscape2.to_vec());
    landscape_type(landscape3.to_vec());
    landscape_type(landscape4.to_vec());
    landscape_type(landscape5.to_vec());
    landscape_type(landscape7.to_vec());
}

fn landscape_type(landscape: Vec<i32>){

    if get_peaks(&landscape){
        println!("mountian\n");
    }else if get_valleys(&landscape) {
        println!("valley\n");
    }else{
        println!("neither\n");
    }
}

fn get_peaks(landscape: &Vec<i32>) -> bool{

    let mut ascending: i32 = 0;
    let mut decending: i32 = 0;

    for (i,x) in landscape.iter().enumerate(){
        if i > 0 && i !=landscape.len(){
            //peak
            if &landscape[i-1] < x {
                ascending += 1;
                if decending > 0 {
                    //println!("not a peak");
                    return false;                }
            }
            if &landscape[i-1] > x {
                decending += 1;
            }
        } 
    }

    if ascending == 0 || decending == 0{
        return false;
    }
    //println!("up: {}, down: {}",ascending,decending);
    true
}

fn get_valleys(landscape: &Vec<i32>) -> bool{

    let mut ascending: i32 = 0;
    let mut decending: i32 = 0;

    for (i,x) in landscape.iter().enumerate(){
        if i > 0 {//&& i !=landscape.len(){
            //valley
            if &landscape[i-1] > x {
                decending += 1;
                if ascending > 0 {
                    //println!("not a valley");
                    return false;
                }
            }
            if &landscape[i-1] < x {
                ascending += 1;
            }
        } 
    }

    if ascending == 0 || decending == 0{
        return false;
    }
    //println!("up: {}, down: {}",ascending,decending);
    true
}