//Used for measuring the average CPU time
#[macro_use]
extern crate timeit;



fn main() {
    println!("{:?}",primes(101));
}

//This prime generator exploits the fact that every prime number is either 6n+1 or 6n-1
//It generates prime numbers up to a certain size
fn primes(len:usize) -> Vec<usize>{
    //Generate an initial list of primes
    let mut primes : Vec<usize> = Vec::new();
    primes.push(2);
    primes.push(3);
    primes.push(5);
    primes.push(7);
    
    let mut n = 2; 
    while primes.len() < len{
        //6n-1
        let option_a = n*6-1;
        let mut valid = true;
        for val in primes.iter(){
            if option_a%val == 0{
                valid=false;
                break;
            }
        }
        if valid{
            primes.push(option_a);
        }

        //6n+1
        let option_b = n*6+1;
        let mut valid = true;
        for val in primes.iter(){
            if option_b%val == 0{
                valid=false;
                break;
            }
        }
        if valid{
            primes.push(option_b);
        }

        //Increase counter
        n=n+1;
    }
    return primes;
}
