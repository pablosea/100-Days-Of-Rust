
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::time::Instant;

fn unique_paths(m: i32, n: i32) -> Option<BigInt> {
    let total_steps = BigInt::from(m + n - 2);
    let k = BigInt::from(m.min(n) - 1);
    (1..=k.clone().to_i32().unwrap()).try_fold(BigInt::one(), |acc, i| {
        let term = total_steps.checked_add(&BigInt::from(i))?
                              .checked_sub(&k)?;
        acc.checked_mul(&term)?.checked_div(&BigInt::from(i))
    })
}

fn original_unique_paths(m: i32, n: i32) -> Option<BigInt> {
    let mut dp = vec![vec![BigInt::from(1); n as usize]; m as usize];
    for i in 1..m as usize {
        for j in 1..n as usize {
            dp[i][j] = dp[i-1][j].clone().checked_add(&dp[i][j-1])?;
        }
    }
    Some(dp[(m-1) as usize][(n-1) as usize].clone())
}

fn compare_unique_paths(m: i32, n: i32) {
    let start = Instant::now();
    let optimized = unique_paths(m, n);
    let optimized_time = start.elapsed();

    let start = Instant::now();
    let original = original_unique_paths(m, n);
    let original_time = start.elapsed();

    println!("Optimized: {:?} in {:?}", optimized, optimized_time);
    println!("Original: {:?} in {:?}", original, original_time);

    // Calculate and print the speed difference
    let speed_diff = if optimized_time > original_time {
        let diff = optimized_time.as_nanos() as f64 / original_time.as_nanos() as f64;
        format!("Optimized is {:.2}x slower", diff)
    } else {
        let diff = original_time.as_nanos() as f64 / optimized_time.as_nanos() as f64;
        format!("Optimized is {:.2}x faster", diff)
    };
    println!("{}", speed_diff);
    println!(); // Add a blank line for readability
}

fn main() {
    println!("{:?}", unique_paths(3, 7));
    println!("{:?}", unique_paths(3, 2));
    println!("{:?}", unique_paths(90, 56));
    compare_unique_paths(3, 7);
    compare_unique_paths(3, 2);
    compare_unique_paths(90, 56);
}
