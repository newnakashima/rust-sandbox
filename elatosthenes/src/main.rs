fn main() {
    sieve1(8190);
}

fn sieve1(n: i32) {
    let mut flag = vec![true; (n+1) as usize];
    let mut p: i32;
    let mut count: i32;

    println!("{:>8}", 2);

    count = 1;
    for i in 0..n {
        if flag[i as usize] {
            p = i + i + 3;
            println!("{:>8}", p);
            for k in ((i+p)..n).step_by(p as usize) {
                flag[k as usize] = false;
            }
            count = count + 1;
        }
    }
    println!("\n{:>8} primes\n", count);
}
