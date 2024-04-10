//Used for measuring the average CPU time
#[macro_use]
extern crate timeit;

use std::env;

const USE_USER_INPUT:bool = true;

fn main() {
    //Choose if use user input
    match USE_USER_INPUT {
        true =>{
            //get the last arg
            let mut last_arg:String =String::new();
            for argument in env::args() {
                last_arg=argument;
            }
            let selected_number=  last_arg.parse::<usize>().unwrap();

            println!("\n\nFor fib({}):",selected_number);
            timeit!({
                fib_function(selected_number);
            });
            println!("{}",fib_function(selected_number));
        },
        false =>  {
            //benchmark
            let mut counter = 1;
            while counter<18  {
            println!("\n\nFor fib({}):",counter);
            timeit!({
                fib_function(counter);
            });
            println!("{}",fib_function(counter));
            counter+=1;  
            }
        }
    }
}

fn fib_function(n:usize) -> u128{
    let mut big_num = 2;
    let mut small_num=1;
    for _ in 1..n{
        let aux = big_num;
        big_num +=small_num;
        small_num=aux;
    }
    return big_num;
}