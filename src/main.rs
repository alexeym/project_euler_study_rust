fn main() {
    let p1 = p1_multiples_of_3_and_5(1000);
    assert!(p1 == 233168, "P1 expected {}, result {}", 233168, p1);

    let p2 = p2_even_fibbonachi_numbers(4000000);
    assert!(p2 == 4613732, "P2 expected {}, result {}", 4613732, p2);

    let p3 = p3_largest_prime_factor(600851475143);
    assert!(p3 == 6857, "P3 expected {}, result {}", 6857, p3);
}

fn p3_largest_prime_factor(mut num: i64) -> i64 {
    loop {
        let orig = num;

        for i in 2..num/2 {
            if num % i == 0 {
                num /= i;
                break;
            }
        }

        if num == orig {
            break;
        }
    }
    
    return num;
}

fn p2_even_fibbonachi_numbers(upper_limit: i32) -> i32 {
    let mut sum = 2;
    let mut x_minus_one = 1;
    let mut x = 2;
    
    loop {
        let temp = x;
        x = x + x_minus_one;
        x_minus_one = temp;
        if x >= upper_limit {
            break;
        }
        if x % 2 == 0 {
            sum += x;
        }
    }
    return sum;
}

fn p1_multiples_of_3_and_5(upper_limit: i32) -> i32 {
    let mut sum = 0;
    for i in 1..upper_limit {
        if i % 3 == 0 || i % 5 == 0 {
            sum+=i;
        }
    }
    return sum;
}
