use primes::{PrimeSet, Sieve};

fn main() {
    let mut total_sum = 0;
    let mut primes_iter = Sieve::new();
    let mut current_prime = 2;
    loop {
        if current_prime<= 2000000{
            total_sum+=current_prime;
        }else{
            break;
        }
        current_prime = primes_iter.find(current_prime+1).1;
        println!("Current prime {}",current_prime);
    }
    println!("{}",total_sum);
}
