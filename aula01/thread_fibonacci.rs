mod fibonacci;

use std::thread;
use std::time::Duration;

use fibonacci::Fibonacci;

pub struct ThreadFibonacci;

impl ThreadFibonacci{
    pub fn new() -> ThreadFibonacci {
        ThreadFibonacci
    }
    
    fn thread_fibonacci(){
        thread::spawn(|| {
        for i in 1..10 {
            let fib = Fibonacci::new();
            fib.print_fibonacci(45);
            thread::sleep(Duration::from_millis(1));
        }
    });
    }
}
