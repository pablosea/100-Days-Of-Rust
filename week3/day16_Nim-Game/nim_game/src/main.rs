fn main() {
    nim_game(2);
    nim_game(3);
    nim_game(4);
}

fn nim_game(num: i32) -> bool{
    if num % 4 >= 1 {true} else {false}
}