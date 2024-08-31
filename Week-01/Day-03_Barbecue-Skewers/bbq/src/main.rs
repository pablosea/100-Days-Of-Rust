fn main() {
    kebab_type([
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
      ]);

    kebab_type([ 
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----"
      ]);
 }

 fn kebab_type(grill: [&str; 5]){

    let mut veggie: i32 = 0;
    let mut meaty: i32 =0;

    for i in grill{
        if i.contains("x"){
            meaty += 1;
        }else{
            veggie += 1;
        }
    }
    println!("[{},{}]", veggie, meaty);
 }

