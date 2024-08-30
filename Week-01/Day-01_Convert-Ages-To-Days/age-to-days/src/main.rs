use std::io;

fn main() {

    loop{
        get_age();
        
        if check_done(){
            println!("thx 4 playing");
            break;
        }
    }
 }

fn get_age(){

    loop{

        let mut input_line = String::new();
        println!("Enter your age:");
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let age:u32 = match input_line.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("not a number!");
                break;
            },
        };
        let totdays = age * 365;
        println!("calculated age to days is: {}", totdays);
        break;
    }

}

fn check_done() -> bool{

    loop{
        let mut close_cmd = String::new();
        println!("Continue?: y/n");
        io::stdin()
            .read_line(&mut close_cmd)
            .expect("Failed to read line");
        let close_cmd = close_cmd.trim();
        
        if close_cmd == "n"{
            break;
        } else if close_cmd == "y"{
            get_age();
            continue;
        } else{
            println!("Innvalid input!");
            continue;
        }
    }
    
    true
}