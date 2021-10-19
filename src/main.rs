use rand::{seq::SliceRandom, thread_rng};

fn main() {
    // Create random number generator entity
    let mut rng = thread_rng();

    // Create array with defined number sequence
    let mut first_arr: [u32; 8] = [10, 80, 3, 19, 14, 7, 5, 12];

    // Create vector of number sequence up to 100
    let mut hundred_vec: Vec<u32> = (1u32..101).collect();
    // Randomize sequence
    hundred_vec.shuffle(&mut rng);

    // Create vector of number sequence up to 1000
    let mut thousand_vec: Vec<u32> = (1u32..1001).collect();
    // Randomize sequence
    thousand_vec.shuffle(&mut rng);

    // Print first array, sort array, then print sorted
    println!("First array of 8 items\n");

    println!("Unsorted:");
    println!("{:?}\n", first_arr);

    let vec_size = first_arr.len();
    quick_sort(&mut first_arr, 0, vec_size - 1);

    println!("Sorted:");
    println!("{:?}\n", first_arr);

    // Print second vec, sort vec, then print sorted
    println!("Second array of 100 random items\n");
    println!("Unsorted:");
    println!("{:?}\n", hundred_vec);

    let vec_size: usize = hundred_vec.len();
    quick_sort(&mut hundred_vec, 0, vec_size - 1);

    println!("Sorted:");
    println!("{:?}\n", hundred_vec);

    // Print second vec, sort vec, then print sorted
    println!("Third array of 1000 items\n");
    println!("Unsorted:");
    println!("{:?}\n", thousand_vec);

    let vec_size: usize = thousand_vec.len();
    quick_sort(&mut thousand_vec, 0, vec_size - 1);

    println!("Sorted:");
    println!("{:?}\n", thousand_vec);
}

fn quick_sort<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) {
    /* Apply quicksort algorithm to containers with integer types. */

    if low < high {
        // Get index of element used in partition function
        let p: usize = partition(arr, low, high);

        // The subtraction of p can overflow occasionally
        // this match checks to make sure that it doesn't happen
        // Rust really likes type-safe operations.
        match p.checked_sub(1) {
            Some(number) => quick_sort(arr, low, number),
            None => quick_sort(arr, low, 0),
        }

        // Not using checked operation here because little chance to overflow
        quick_sort(arr, p + 1, high);
    }
}

fn partition<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) -> usize {
    /* Partition container within index bounds, using the last element
        as the partition
    */

    // Get pivot value and location of the current smallest number
    let pivot = arr[high];
    let mut smallest_index = low;

    // Iterate up to the partition index, move values smaller than
    // partition below the climbing "smallest number" index
    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(j, smallest_index);
            smallest_index += 1;
        }
    }
    
    // Place partition value at "smallest number" index
    arr.swap(high, smallest_index);

    // Return the location index of the partition value.
    return smallest_index;
}
