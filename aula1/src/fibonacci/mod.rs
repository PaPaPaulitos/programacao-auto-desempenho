use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

pub struct Fibonacci;

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci {}
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

pub struct ThreadFibonacci;

impl ThreadFibonacci {
    pub fn new() -> ThreadFibonacci {
        ThreadFibonacci
    }
    
    pub fn thread_fibonacci(&self) -> Vec<JoinHandle<()>> {
        let mut handles = Vec::new();
        for _ in 1..10 {
            let handle = thread::spawn(|| {
                let fib = Fibonacci::new();
                fib.print_fibonacci(45);
                thread::sleep(Duration::from_millis(1));
            });
            handles.push(handle);
        }
        handles
    }
}