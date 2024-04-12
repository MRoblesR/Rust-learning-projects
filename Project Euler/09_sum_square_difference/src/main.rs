fn main() {
    println!("Total: {}",square_of_sums(100)-sum_of_squares(100));
}

fn square_of_sums(n:usize) -> usize{
    let total_sum = n*(n+1)/2;
    return total_sum.pow(2);
}

fn sum_of_squares(n:usize) ->usize{
    let mut total_sum = 0;
    for i in 1..n+1{
        total_sum+=i.pow(2);
    }
    return total_sum;
}