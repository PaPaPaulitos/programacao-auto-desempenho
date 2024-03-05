mod fibonacci;

use fibonacci::ThreadFibonacci;

fn main() {
    let fib = ThreadFibonacci::new();
    let handles = fib.thread_fibonacci();

    for handle in handles {
        handle.join().unwrap();
    }
}
