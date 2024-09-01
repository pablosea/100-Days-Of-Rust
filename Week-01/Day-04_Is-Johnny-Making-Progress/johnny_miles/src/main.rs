fn main() {
    progress_days(vec![3, 4, 1, 2]);
    progress_days(vec![10, 11, 12, 9, 10]);
    progress_days(vec![6, 5, 4, 3, 2, 9]);
    progress_days(vec![9, 9]);
}

fn progress_days(miles: Vec<i32>){
    let mut prog_day: i32 =0;
    for (x,i) in miles.iter().enumerate(){
        if x > 0{
            if i > &miles[x-1]{
                prog_day +=1;
            }
        }
    }
    println!("{} total progress days.", prog_day);
}
