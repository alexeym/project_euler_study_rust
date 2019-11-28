fn main() {
    let p1 = p1_multiples_of_3_and_5(1000);
    assert!(p1 == 233168, "P1 expected {}, result {}", 233168, p1);

    let p2 = p2_even_fibbonachi_numbers(4000000);
    assert!(p2 == 4613732, "P2 expected {}, result {}", 4613732, p2);

    let p3 = p3_largest_prime_factor(600851475143);
    assert!(p3 == 6857, "P3 expected {}, result {}", 6857, p3);

    let p4 = p4_largest_palindrome_product();
    assert!(p4 == 906609, "P4 expected {}, result {}", 906609, p4);
}

fn is_palindrome(num: i32) -> bool {
    let mut copy = num;
    let mut num_reversed = 0;
    loop {
        num_reversed += copy % 10;
        copy = copy / 10;
        if copy == 0 {
            break;
        }
        num_reversed *= 10;
    }    
    return num_reversed == num;
}

fn p4_largest_palindrome_product() -> i32 {
    let mut max = -1;
    for i in (99..999).rev() {
        for j in (99..999).rev() {
          let possible_palindrome = i * j;
          if possible_palindrome < max {
            break;
            }
            if is_palindrome(possible_palindrome) {
                max = possible_palindrome;
                break;
            }
        }
    }

    return max;
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
