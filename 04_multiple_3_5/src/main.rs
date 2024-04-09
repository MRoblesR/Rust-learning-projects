
//This problem is stated on https://projecteuler.net/problem=1
fn main() {
    println!("{}", sum_multiples(16));
    println!("{}", sum_multiples(31));
    println!("{}", sum_multiples(1000));

}

fn sum_multiples(max_num:usize) -> usize{

    let mut current_diff = 0;
    let mut accumulated_num = 0;
    let mut accumulated_sum = 0;
    while accumulated_num < max_num{
        accumulated_num = match current_diff{
            0 => accumulated_num + 3,
            1 => accumulated_num + 2,
            2 => accumulated_num + 1,
            3 => accumulated_num + 3,
            4 => accumulated_num + 1,
            5 => accumulated_num + 2,
            6 => accumulated_num + 3,
            _ => panic!("This is wrong")
        };
        accumulated_sum=accumulated_sum+accumulated_num;
        current_diff= ( current_diff+1) %7;
    }
    return accumulated_sum-accumulated_num;
}

