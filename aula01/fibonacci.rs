pub struct Fibonacci;

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci
    }

    fn fibonacci(n: u32) -> u32 {
        if n <= 1 {
            n
        } else {
            Self::fibonacci(n - 1) + Self::fibonacci(n - 2)
        }
    }

    pub fn print_fibonacci(&self, n: u32) {
        for i in 0..n {
            println!("Fibonacci: {}", Self::fibonacci(i));
        }
    }
}

