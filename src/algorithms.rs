// All the sorting algorithms in the crate.

pub fn bubble_sort(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    // Bubble sort algorithm

    let mut num_index = 0; // The index in the vector that the program is reading.

    loop {
        if num_index > vec.len() - 2 { // Checks if the index is the second last number (If this is true, the vector is sorted).
            break;                     // NOTE: Vec<i32>.len() returns the length of the vec starting at 1 instead of 0.
        }

        if vec[num_index] > vec[num_index + 1] {
           let _ = vec.swap(num_index, num_index + 1); // Swaps the value of the current index with the value of the index one after it.
           num_index = 0                               // Checks the first index again.
        }
        
        else {
            num_index = num_index + 1; // If the value of num_index is less than the value of num_index + 1, the index moves up one position.
        }
    }

    return vec; // Despite the name
}

pub fn count_sort(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    // Counts the number of each value inside the list
    // and constructs a new list containing those values.

    // This sounds like a very inefficient sorting algorithm.

    return vec;
}

pub fn bogo_sort(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    // Scrambles the order of the values until the vec is sorted.
    
    return vec;
}
