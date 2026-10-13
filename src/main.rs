fn main() {
    for i in 1..=100 {
        let result = fizzbuzz2(i);
        println!("{result}");
    }
}

#[allow(dead_code)]
fn fizzbuzz(num: u32) -> String {
    if num % 15 == 0 {
        "FizzBuzz".to_string()
    } else if num % 3 == 0 {
        "Fizz".to_string()
    } else if num % 5 == 0 {
        "Buzz".to_string()
    } else {
        num.to_string()
    }
}

fn fizzbuzz2(num: u32) -> String {
    // Using match 
    match num {
        n if n.is_multiple_of(15) => "FizzBuzz".to_string(),
        n if n.is_multiple_of(3) => "Fizz".to_string(),
        n if n.is_multiple_of(5) => "Buzz".to_string(),
        _ => num.to_string(),
    }
}
