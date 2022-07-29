use rand::{thread_rng, Rng};

fn main() {
    println!("Bubble Sort Algorythm");

    let mut unsorted_vec: Vec<i32> = make_vec(100); // Creates a random unsorted vec
    println!("Unsorted vec = {unsorted_vec:?}"); // Prints the unsorted vec to console

    let sorted_vec = bubble_sort(&mut unsorted_vec); // Sorts the unsorted vec
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

fn bubble_sort(unsorted_vec:&mut Vec<i32>) -> &mut Vec<i32> {
    let mut num_index = 0;

    loop {
        if num_index > &unsorted_vec.len() - 2 {
            break;
        }

        if &unsorted_vec[num_index] > &unsorted_vec[num_index + 1] {
           let _ = &unsorted_vec.swap(num_index, num_index + 1); // Adding "let _ = " part because rust compiler is throwing a warning
           num_index = 0
        }
        
        else {
            num_index = num_index + 1;
        }
    }

    return unsorted_vec;
}
