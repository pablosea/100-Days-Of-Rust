
fn main() {

    let mut nums1: [i32; 6] = [1,2,3,0,0,0];
    let nums2: [i32; 3] = [2,5,6];

    let mut nums3: [i32;10] = [-76,-98,2,8,999,0,0,0,0,0];
    let nums4: [i32;5] = [7,1000,453,2,0];

    let mut nums5: [i32; 10] = [1,2,3,0,0,0,0,0,0,0];
    let nums6: [i32; 5] = [2,5,6,9,43];

    merge_arrays(&nums1, &nums2);
    merge_arrays(&nums3, &nums4);
    merge_arrays(&nums5, &nums6);

    //second solution is to copy from slice, keeing output an array
    nums1[3..].copy_from_slice(&nums2);
    nums3[5..].copy_from_slice(&nums4);
    nums5[5..].copy_from_slice(&nums6);

    println!("\n {:?} \n {:?} \n {:?}",nums1, nums3, nums5);

}

//first solution makes them vectors
//input arrays can be any length
fn merge_arrays(ary1: &[i32], ary2: &[i32]) {
    
    let (mut asvec1, mut asvec2) = (ary1.to_vec(), ary2.to_vec());

    remove_zeros(&mut asvec1);
    remove_zeros(&mut asvec2);
   
    asvec1.append(&mut asvec2);

    asvec1.sort();
    println!("{:?}", asvec1);
}

fn remove_zeros(new_vector: &mut Vec<i32>){
    new_vector.retain(|&x| x!= 0);
}
