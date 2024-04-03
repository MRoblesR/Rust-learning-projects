//Used for measuring the average CPU time
#[macro_use]
extern crate timeit;

use num_bigint::BigUint;

fn main() {
    print!("{}",factorial_u128(35));
    for i in 0..100{
        timeit!({
            factorial_big_int(i);
        });
        println!("The factorial of {} equals to {}",i,factorial_big_int(i))
    }
}

fn factorial_big_int(num:u128) -> BigUint{
    let mut current_num = BigUint::new(vec![1]);
    match num {
        0 | 1 => return current_num,
        _ =>{
            for i in 1..num+1{
                current_num = current_num * i;
            }
        }
    }
    return current_num;
}

fn factorial_u128(num:u128) -> u128{
    let mut current_num = 1;
    match num {
        0 | 1 => return current_num,
        _ =>{
            for i in 1..num+1{
                current_num =  current_num * i;
            }
        }
    }
    return current_num;
}