use rand::{thread_rng, Rng};

pub mod algorithms;

fn main() {
    println!("Bubble Sort Algorythm");

    let mut unsorted_vec: Vec<i32> = make_vec(100); // Creates a random unsorted vec
    println!("Unsorted vec = {unsorted_vec:?}"); // Prints the unsorted vec to console

    let sorted_vec = algorithms::bubble_sort(&mut unsorted_vec); // Sorts the unsorted vec
    println!("Sorted vec =   {sorted_vec:?}"); // Prints the sorted vec to the console
}

fn make_vec(list_length: i32) -> Vec<i32> {
    let mut unsorted_vec = Vec::new();
    let mut rng = thread_rng();
    
    for _ in 0..list_length {
        unsorted_vec.push(rng.gen_range(-9999..=9999));
    }
    
    return unsorted_vec;
}
