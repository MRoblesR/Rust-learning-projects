use std::collections::HashMap;
use primes::factors_uniq;

fn main() {
    let mut nums = Vec::new();
    for i in 1..21{
        nums.push(i);
    }
    println!("{:?}",smallest_multiple(nums));
}

fn smallest_multiple(nums :Vec<u64>) -> u64{
    let mut map:HashMap<u64, u32> = HashMap::new();
    for num in nums{
        for prime in factors_uniq(num){
            let new_exponent = get_exponent(num, prime);
            match map.get(&prime) {
                Some(value) => {
                    if new_exponent>*value{
                        map.insert(prime, new_exponent);
                    }
                },
                None => {map.insert(prime, new_exponent);}
            }
        }
    }
        let mut result=1;
    for pair in map.iter(){
        let prime = pair.0;
        let exponent  = pair.1;
        result*= prime.pow(*exponent);
    }

    return result;
}

fn get_exponent(evaluated_number:u64,prime_number:u64) -> u32{
    let mut exponent=0;
    while evaluated_number%prime_number.pow(exponent+1)==0 {
        exponent+=1;
    }
    return exponent;
}