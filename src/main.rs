use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let mut rng = thread_rng();

    // let mut sorting_arr: [usize; 1000] = [0_usize; 1000];
    let mut sorting_vec: Vec<u32> = (1u32..101).collect();
    sorting_vec.shuffle(&mut rng);

    println!("{:?}", sorting_vec);

    let vec_size = sorting_vec.len();
    quick_sort(&mut sorting_vec, 0, vec_size - 1);

    println!("{:?}", sorting_vec);

    for i in 0..vec_size - 2 {
        assert!(sorting_vec[i] <= sorting_vec[i + 1])
    }
}

fn quick_sort<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let p: usize = partition(arr, low, high);

        // The subtraction of p can overflow occasionally
        // this match checks to make sure that it doesn't happen
        match p.checked_sub(1) {
            Some(number) => quick_sort(arr, low, number),
            None => quick_sort(arr, low, 0),
        }

        quick_sort(arr, p + 1, high);
    }
}

fn partition<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) -> usize {
    // Get pivot value and location of smallest number "right now"
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

    return smallest_index;
}
