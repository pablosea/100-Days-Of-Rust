fn main() {
   next_prime(12);
   next_prime(24);
   next_prime(11);
   next_prime(100);
   next_prime(1000062);
   next_prime(100000000200000070);
// next_prime(100000000200000000084); this returns 100000000200000000151 which is prime but takes like 8 minutes to run lol
}

fn is_prime(num:i128)-> bool{
    let mut factors: Vec<i128> = Vec::new();
    let mut i:i128 = 1;
    while i*i <= num {
        if num % i == 0  {
            factors.push(i);
        }
        if factors.len() > 1{
            return false;
        }
        i += 1;
    }
    return true;
}

fn next_prime(num: i128) -> i128{
    let mut prime_finder: i128 = 1;
    if is_prime(num) == true{
        println!("{} is a prime!", num);
        num
    }else {
        while is_prime(num + prime_finder) == false{
            prime_finder += 1;
        } 
        println!("{} is not a prime, the next prime is {}", num, num + prime_finder);
        num + prime_finder
    }
}