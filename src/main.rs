fn check_prime(num:i32) -> bool{
    if num == 1 {
        return false;
    } else if num == 2 || num == 3 {
        return  true;
    } else if num%2 == 0 || num%3 == 0 {
        return  false;
    } else {
        let mut i:i32 = 5;
        while i*i <= num {
            if num%i == 0 || num%(i+2) == 0 {
                return  false;
            }
            i += 6;
        }
        return true;
    }
}

fn main() {
    println!("{}",check_prime(29));
}
