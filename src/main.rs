#[warn(non_snake_case)]
// Insertion Sort
// Best Case TC: O(n)
// Worst Case TC: O(n^2)

use std::time::Instant;

pub fn insertion_sort(arr: &mut [i32]) {
        let now = Instant::now();
        let mut j;
        for i in 1..arr.len() {
            j = i;
            while j > 0 && arr[j-1] > arr[j] {
                arr.swap(j-1, j);
                j -= 1;
            }
        }
        let elapsed = now.elapsed();
        println!("Insertion Sort Time taken {:?}", elapsed);
    }

fn main() {
    let mut number = [6,5,3,2,1];
    insertion_sort(&mut number);
    println!("Sorted Array {:?}", number);
}