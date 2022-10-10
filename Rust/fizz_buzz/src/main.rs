use fizz_buzz::fizzbuzz;
fn main() {
    let mut x = 1;
    loop {
        if x > 40 {
            break;
        }
        println!("Fizzbuzz change {} to {}", x, fizzbuzz(x));
        x = x + 1;
    }
}