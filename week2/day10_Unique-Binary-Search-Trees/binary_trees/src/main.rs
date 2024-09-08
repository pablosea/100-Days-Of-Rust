

fn catalan(num: i128) -> i128{

    if num <= 1{ return 1}
    (0..num).map(|i| catalan(i)* catalan(num-1-i)).sum()

}

fn c(n:i128)->i128{(2..=n).fold(1,|a,k|a*(4*k-2)/(k+1))}

fn main(){

    println!("Unique trees: {}", catalan(3));
    println!("Unique trees: {}", c(3));

}