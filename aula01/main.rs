mod fibonacci;

use fibonacci::Fibonacci;

fn main() {
    let fib = Fibonacci::new();
    fib.print_fibonacci(10);
}
