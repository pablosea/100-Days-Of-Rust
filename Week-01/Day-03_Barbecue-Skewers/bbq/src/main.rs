use std::array;



fn main() {
    kebab_type([
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
      ].to_vec());

    kebab_type([ 
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----"
      ].to_vec());
 }

 fn kebab_type(grill: Vec<&str>){

    let mut veggie: i32 = 0;
    let mut meaty: i32 =0;

    for i in grill{
        i.replace(&['-','"','!'][..],"");
        if i.contains("x"){
            meaty += 1;
        }else{
            veggie += 1;
        }
    }
    println!("[{},{}]", veggie, meaty);
 }

