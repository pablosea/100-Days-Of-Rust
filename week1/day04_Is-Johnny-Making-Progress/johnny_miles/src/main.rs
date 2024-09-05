fn main() {
    progress_days(&[3, 4, 1, 2]);
    progress_days(&[10, 11, 12, 9, 10]);
    progress_days(&[6, 5, 4, 3, 2, 9]);
    progress_days(&[9, 9]);
}

fn progress_days(miles: &[i32]){
    let mut prog_day: i32 =0;
    for (x,i) in miles.iter().enumerate(){
        if x > 0{
            //Original code line: if i > miles[x-1]{
            //A friend writing the same program got suggested "help: consider dereferencing the borrow: if *i > miles[x-1]{"
            //I got suggested "consider borrowing here: if i > &miles[x-1] "
            //Why does derefrenced "i" work just the same as refrenced "miles"? 
            //Why does rust complier not suggest the same thing?
            if *i > miles[x-1]{
                prog_day +=1;
            }
        }
    }
    println!("{} total progress days.", prog_day);
}
