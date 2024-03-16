mod mergesort;

use mergesort::MergeSort;

fn main() {
    let data = vec![4, 2, 3, 1];
    println!("Initial Data: {:?}", data);
    let sorter = MergeSort::new(data);
    let sorted_data = sorter.sort();
    println!("Final Data: {:?}", sorted_data);
}
