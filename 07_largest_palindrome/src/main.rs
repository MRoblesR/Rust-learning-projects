use std::{borrow::BorrowMut, cmp::Reverse};

fn main() {
    for i in 900..1000{
        for j in 900..1000{
            if is_palindrome(i*j){
                println!("{}",i*j);
            }
        }
    }
}


fn is_palindrome(x:isize) -> bool{
    return x-reverse_number(x)==0;
}

fn reverse_number(x:isize) -> isize{
    let mut x = x.clone(); //Shadowing
    let mut new_num = 0;
    while x>=10 {
        new_num=new_num*10+x%10;
        x/=10;
    }
    return new_num*10+x;
}