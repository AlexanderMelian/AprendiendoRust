fn is_prime(n : i32) -> bool{
    if n == 0 || n == 1{
        return false;
    }  
    if n == 2 || n == 3 || n == 5 || n == 7{
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0{
        return false;
    }
    return true;
}

fn main(){
    let max : i32 = 100;
    let mut i : i32 = 0;
    let mut count : i32 = 0;
    while i<max {
        if is_prime(i) {
            count += 1;to
        }
        i +=1 ;
    }
    println!("{}",count);
}