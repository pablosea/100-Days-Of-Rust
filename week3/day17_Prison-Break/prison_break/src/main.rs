

fn main() {
    freed_prisoners(vec![1, 1, 0, 0, 0, 1, 0]);
    freed_prisoners(vec![1, 1, 1]);
    freed_prisoners(vec![0, 0, 0]);
    freed_prisoners(vec![0, 1, 1, 1]);
}

fn freed_prisoners(cell_block: Vec<i32>) -> i32{

    let swapped_block: Vec<i32> = cell_block.iter().map(|i| 1-i).collect();

    let mut free_ppl = 0;
    let mut cell_index = 0;
    let length = cell_block.len();
    let mut switch = true;

    if cell_block[0] == 0{
        println!("Locked in! Free people: {}", free_ppl);
        return 0;
    }

    while cell_index < length{

        if switch{
            if cell_block[cell_index] == 1{
                free_ppl += 1;
                switch = false;
            }
        }else{
            if swapped_block[cell_index] == 1{
                free_ppl += 1;
                switch = true;
            }
        }
        cell_index += 1;
    }

    println!("{:?} \n{:?} \nFree people: {}", cell_block, swapped_block, free_ppl);
    free_ppl
}

