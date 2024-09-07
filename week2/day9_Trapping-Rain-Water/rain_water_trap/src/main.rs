use std::collections::HashMap;


fn main() {

    let elevation_map1: [i32; 12] = [0,1,0,2,1,0,1,3,2,1,2,1];
    let elevation_map2: [i32; 6] = [4,2,0,3,2,5];
    let elevation_map3: [i32; 20] = [0,0,0,3,7,0,5,4,0,9,0,3,5,6,12,7,9,2,0,3];
    let elevation_map4: [i32; 14] = [4,2,0,3,2,5,0,0,7,0,2,0,0,0];

    catch_water(elevation_map1.to_vec());
    catch_water(elevation_map2.to_vec());
    catch_water(elevation_map3.to_vec());
    catch_water(elevation_map4.to_vec());


 //   v2(elevation_map1.to_vec());
}

//find the largest height in an array
//create 2D area of len * height of a given vec
//stack n vectors of n length to make the area
//populate the vectors with 2 for ground 1 for water and 0 for air
//count the 1's

fn catch_water(somevec: Vec<i32>){
    let playvec: Vec<i32> = somevec;
    
    //get h, l, a
    let height: &i32 = playvec.iter().max().unwrap();
    let length: i32 = playvec.len() as i32;
    let area: i32 = length * height;

    //make vector of vectors
    let mut mapvec: Vec<Vec<i32>> = vec![vec![0; length as usize]; *height as usize];

    //for each ground value in "elevation" vector, update the value in map vec
    //increment via height 
    let mut height_index_counter: i32 = *height;
    let mut water: i32 = 0;

    //loop through the vector of vectors and update points to "2" where the height matches the original playvec
    for (x,level) in &mut mapvec.clone().iter().enumerate() {
        for (y, mut point) in level.iter().enumerate(){
            if playvec[y] == height_index_counter{
                point = &2;
                mapvec[x].remove(y);
                mapvec[x].insert(y, *point);                   
            }
            //check to see if we are past index 0
            if x > 0{
                //check if this position in the previous vector has a "2", if so, add a "2" here
                if &mapvec[x-1][y] == &2{
                    point = &2;
                    mapvec[x].remove(y);
                    mapvec[x].insert(y, *point);
                }
            }  
        }
        height_index_counter -= 1;
    }

    //loop thought the vector of vectors 
    //if a 2 comes before and after a point, update that point to 1 to represent water
    //count the water
    for (x, level) in &mut mapvec.clone().iter().enumerate(){
        for (y, mut point) in level.iter().enumerate(){
            if point == &0 && level[..y].contains(&2) && level[y..].contains(&2){
                point = &1;
                mapvec[x].remove(y);
                mapvec[x].insert(y, *point);
                water += 1;
            }
        }
    }

    println!("height: {}, length: {}, area: {}\nMap:",height, length, area);
    for i in &mapvec{
        println!("{:?}", i);
    }
    println!("\nTotal water caught: {} \n\n",water);
}


//second attempt at a more refined version
//fn v2(somevec: Vec<i32>){
//
//    let mut playvec: Vec<i32> = somevec;
//    let mut water = 0;
//    let mut data: HashMap<&str, i32> = HashMap::new();
//    data.insert("wall1",0);
//    data.insert("wall2", 0);
 //   let mut d1 = data.get("wall1").copied().unwrap();
//
//    for (x,i) in playvec.iter().enumerate(){
//        if data.get("wall1").copied().unwrap() < *i {
//            data.insert("wall1", *i);
//        }
//        
//        if x > 0 && x < playvec.len()-1{
//            if playvec[x+1] > data.get("wall1").copied().unwrap(){
//                data.insert("wall2", playvec[x+1]);
//            }else if d1 != 0{
//
 //               water = data.get("wall1").copied().unwrap() - i;
//            }
//        }
//
//        [w1..w2].iter()
        
//    }

 //   println!("{}", water);
    //update height with the vec number
    //get the length of the index between this number and the next biggest number 
    //at each point in that index, subtract the value from the lesser of the two walls
    
//}